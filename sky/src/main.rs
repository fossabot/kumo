#[macro_use]
extern crate actix_web;
extern crate askama;

use std::env;

use actix_files as fs;
use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use askama::Template;

mod proto;

fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let sys = actix_rt::System::new("sky");

    HttpServer::new(move || {
        // start http server
        App::new()
            .service(favicon)
            .service(web::resource("/").route(web::get().to(index)))
            .default_service(
                // 404 for GET requests
                web::resource("")
                    .route(web::get().to(not_found))
                    // all requests that are not GET
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(|| HttpResponse::MethodNotAllowed()),
                    ),
            )
    })
    .bind("127.0.0.1:8080")?
    .start();

    println!("Starting http server: 127.0.0.1:8080");
    sys.run()
}

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

fn index(req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);
    let body = Index.render().unwrap();

    HttpResponse::Ok().content_type("text/html").body(body)
}

// favicon handler
#[get("/favicon.ico")]
fn favicon(req: HttpRequest) -> Result<fs::NamedFile> {
    println!("{:?}", req);
    Ok(fs::NamedFile::open("static/favicon.ico")?)
}

#[derive(Template)]
#[template(path = "404.html")]
struct NotFoundTemplate<'a> {
    message: &'a str,
}

// 404 not found handler
fn not_found(req: HttpRequest) -> HttpResponse {
    println!("{:?}", req);

    let body = NotFoundTemplate {
        message: "Not Found",
    }
    .render()
    .unwrap();

    HttpResponse::NotFound()
        .content_type("text/html")
        .body(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};
    use futures::future::IntoFuture;

    #[test]
    fn test_index() {
        let req = test::TestRequest::get().uri("/").to_http_request();
        let resp = test::block_on(index(req).into_future()).unwrap();

        assert_eq!(resp.status(), http::StatusCode::OK);
    }

    #[test]
    fn test_404() {
        let req = test::TestRequest::get().uri("dont_exist").to_http_request();
        let resp = test::block_on(not_found(req).into_future()).unwrap();

        assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
    }
}
