extern crate bytes;
extern crate env_logger;
extern crate futures;
#[macro_use]
extern crate log;
extern crate prost;
extern crate tokio;
extern crate tower_grpc;
extern crate tower_hyper;

pub mod compute {
    include!(concat!(env!("OUT_DIR"), "/compute.rs"));
}

use std::env;

use compute::{server, Compute, ComputeListRequest, ComputeListResponse};

use futures::{future, Future, Stream};
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_grpc::{Request, Response};
use tower_hyper::server::{Http, Server};

extern crate rusoto_core;
extern crate rusoto_ec2;

use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Filter, Reservation};

#[derive(Clone, Debug)]
struct Service;

impl server::ComputeService for Service {
    type ListFuture = future::FutureResult<Response<ComputeListResponse>, tower_grpc::Status>;

    fn list(&mut self, request: Request<ComputeListRequest>) -> Self::ListFuture {
        println!("REQUEST = {:?}", request);

        let response = Response::new(ComputeListResponse {
            computes: list_computes(),
        });

        future::ok(response)
    }
}

pub fn main() {
    let _ = ::env_logger::init();

    let port;
    match env::var("PORT") {
        Ok(val) => port = val,
        Err(_) => port = "10100".to_string(),
    }

    let new_service = server::ComputeServiceServer::new(Service);

    let mut server = Server::new(new_service);

    let http = Http::new()
        .http2_only(true)
        .executor(DefaultExecutor::current())
        .clone();

    let addr = &format!("[::]:{}", port).parse().unwrap();
    let bind = TcpListener::bind(&addr).expect("bind");
    println!("Listening on http://{}", addr);

    let serve = bind
        .incoming()
        .for_each(move |sock| {
            if let Err(e) = sock.set_nodelay(true) {
                return Err(e);
            }

            let serve = server.serve_with(sock, http.clone());
            tokio::spawn(serve.map_err(|e| error!("h2 error: {:?}", e)));

            Ok(())
        })
        .map_err(|e| eprintln!("accept error: {}", e));

    tokio::run(serve)
}

pub fn list_computes() -> Vec<Compute> {
    let client = Ec2Client::new(Region::UsWest2);

    let input = ec2_describe_input();
    let computes: Vec<Compute> = vec![];
    let mut result: Vec<Compute> = vec![];

    match client.describe_instances(input).sync() {
        Ok(resp) => match resp.reservations {
            Some(reservations) => {
                // println!("{}", reservations.len());
                result = normalize_reservations(reservations, computes);
            }
            None => println!("No instances to show"),
        },
        Err(_) => {
            println!("Error");
        }
    }

    result
}

// https://rusoto.github.io/rusoto/rusoto_ec2/struct.DescribeInstancesRequest.html
pub fn ec2_describe_input() -> DescribeInstancesRequest {
    let dry_run = Some(false);
    let filters = Some(vec![
        // https://rusoto.github.io/rusoto/rusoto_ec2/struct.Filter.html
        Filter {
            name: Some(String::from("instance-state-name")),
            values: Some(vec![String::from("running")]),
        },
    ]);
    let instance_ids = None;
    let max_results = None;
    let next_token = None;

    DescribeInstancesRequest {
        dry_run: dry_run,
        filters: filters,
        instance_ids: instance_ids,
        max_results: max_results,
        next_token: next_token,
    }
}

// https://rusoto.github.io/rusoto/rusoto_ec2/struct.DescribeInstancesResult.html
// https://rusoto.github.io/rusoto/rusoto_ec2/struct.Instance.html
pub fn normalize_reservations(
    reservations: Vec<Reservation>,
    mut result: Vec<Compute>,
) -> Vec<Compute> {
    for rev in reservations {
        let computes = rev.instances.unwrap();

        for comp in computes {
            result.push(Compute {
                id: comp.instance_id.unwrap(),
                compute_type: comp.instance_type.unwrap(),
            });
        }
    }

    result
}
