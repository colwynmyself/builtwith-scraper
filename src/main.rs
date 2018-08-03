#[macro_use]
extern crate diesel;

extern crate futures;
extern crate hyper;
extern crate queryst;

mod util;
mod db;

use futures::future;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use queryst::parse;
use util::http;

type BoxFuture = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn router(req: Request<Body>) -> BoxFuture {
    let mut response = Response::new(Body::empty());
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("GET on /domain?domain=somedomain.com");
        }

        (&Method::GET, "/domain") => {
            let query = parse(req.uri().query().unwrap()).unwrap();
            let mut domain: String = query["domain"].to_string();

            if domain == "null" {
                *response.body_mut() = Body::from("'domain' is a required query parameter");
            } else {
                domain = domain.replace("\"", "");
                println!("Domain requested: {}", domain);
                let uri = http::build_builtwith_hyper_uri(domain);

                let res = http::request(uri).wait();
                let body = res.unwrap();

                *response.body_mut() = body;
            }
        }

        (&Method::GET, "/domain/requests") => {
            let connection = db::lib::connect();
            db::queries::read_requests(&connection);
        }

        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
            *response.body_mut() = Body::from("404");
        }
    };

    Box::new(future::ok(response))
}

fn main() {
        let addr = ([127, 0, 0, 1], 8000).into();

    let server = Server::bind(&addr)
        .serve(|| service_fn(router))
        .map_err(|e| eprintln!("server error: {}", e));

    println!("Listening on http://{}", addr);

    rt::run(server);
}
