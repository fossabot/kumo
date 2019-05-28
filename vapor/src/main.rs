#[macro_use]
extern crate log;

use env_logger;
use futures::{future, Future, Stream};
use std::env;
use tokio::executor::DefaultExecutor;
use tokio::net::TcpListener;
use tower_grpc::{Request, Response};
use tower_hyper::server::{Http, Server};

mod aws;
mod protobuf;

use protobuf::compute::{server, ComputeListRequest, ComputeListResponse};

#[derive(Clone, Debug)]
struct Service;

impl server::ComputeService for Service {
    type ListFuture = future::FutureResult<Response<ComputeListResponse>, tower_grpc::Status>;

    fn list(&mut self, request: Request<ComputeListRequest>) -> Self::ListFuture {
        println!("REQUEST = {:?}", request);

        let response = Response::new(ComputeListResponse {
            computes: aws::list_computes(),
        });

        future::ok(response)
    }
}

fn main() {
    let _ = env_logger::init();

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
    println!("Listening on tcp://{}", addr);

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
