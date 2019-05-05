extern crate rusoto_core;
extern crate rusoto_ec2;

use std::env;

use rusoto_core::Region;
use rusoto_ec2::{DescribeInstancesRequest, Ec2, Ec2Client, Filter, Reservation};

use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};

use serde::{Deserialize, Serialize};

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let sys = actix_rt::System::new("vapor");

    HttpServer::new(move || {
        // start http server
        App::new().service(web::resource("/").route(web::get().to(index)))
    })
    .bind("127.0.0.1:8081")?
    .start();

    println!("Starting vapor server on 127.0.0.1:8081");
    sys.run()
}

fn index(_req: HttpRequest) -> HttpResponse {
    let computes = list_computes();

    HttpResponse::Ok().json(computes)
}

fn list_computes() -> Vec<Compute> {
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
fn ec2_describe_input() -> DescribeInstancesRequest {
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

#[derive(Debug, Serialize, Deserialize)]
struct Compute {
    id: String,
    compute_type: String,
}

// https://rusoto.github.io/rusoto/rusoto_ec2/struct.DescribeInstancesResult.html
// https://rusoto.github.io/rusoto/rusoto_ec2/struct.Instance.html
fn normalize_reservations(
    reservations: Vec<Reservation>,
    mut result: Vec<Compute>,
) -> Vec<Compute> {
    for rev in reservations {
        let computes = rev.instances.unwrap();

        for compute in computes {
            result.push(Compute {
                id: compute.instance_id.unwrap(),
                compute_type: compute.instance_type.unwrap(),
            });
        }
    }

    result
}
