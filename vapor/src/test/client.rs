extern crate bytes;
extern crate env_logger;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate log;
extern crate prost;
extern crate tokio;
extern crate tower_grpc;
extern crate tower_hyper;
extern crate tower_request_modifier;
extern crate tower_service;
extern crate tower_util;

use std::env;

use futures::Future;
use hyper::client::connect::{Destination, HttpConnector};
use tower_grpc::Request;
use tower_hyper::{client, util};
use tower_util::MakeService;

pub mod compute {
    include!(concat!(env!("OUT_DIR"), "/compute.rs"));
}

pub fn main() {
    let _ = ::env_logger::init();

    let port;
    match env::var("PORT") {
        Ok(val) => port = val,
        Err(_) => port = "10100".to_string(),
    }

    let uri: http::Uri = format!("http://[::1]:{}", port).parse().unwrap();

    let dst = Destination::try_from_uri(uri.clone()).unwrap();
    let connector = util::Connector::new(HttpConnector::new(4));
    let settings = client::Builder::new().http2_only(true).clone();
    let mut make_client = client::Connect::new(connector, settings);

    let list = make_client
        .make_service(dst)
        .map(move |conn| {
            use compute::client::ComputeService;

            let conn = tower_request_modifier::Builder::new()
                .set_origin(uri)
                .build(conn)
                .unwrap();

            ComputeService::new(conn)
        })
        .and_then(|mut client| {
            use compute::ComputeListRequest;

            client
                .list(Request::new(ComputeListRequest {}))
                .map_err(|e| panic!("gRPC request failed; err={:?}", e))
        })
        .and_then(|response| {
            println!("RESPONSE = {:?}", response);
            Ok(())
        })
        .map_err(|e| {
            println!("ERR = {:?}", e);
        });

    tokio::run(list);
}
