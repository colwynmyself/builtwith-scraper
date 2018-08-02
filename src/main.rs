extern crate config;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate queryst;

use futures::future;
use hyper::Client;
use hyper::rt::{self, Future};
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use hyper_tls::HttpsConnector;
use queryst::parse;

type BoxFuture = Box<Future<Item = Response<Body>, Error = hyper::Error> + Send>;

fn build_builtwith_hyper_uri(domain: String) -> hyper::Uri {
    let mut config = config::Config::default();

    config
        .merge(config::File::with_name("Config")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();

    let builtwith_key: String = config.get_str("builtwith_key").unwrap();
    let url: String = format!("https://api.builtwith.com/free1/api.json?KEY={key}&LOOKUP={url}", key = builtwith_key, url = domain);

    url.parse().unwrap()
}

fn request(url: hyper::Uri) -> impl Future<Item=Body, Error=hyper::Error> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client: Client<_, hyper::Body> = Client::builder()
        .build::<_, hyper::Body>(https);

    let res_future = client.get(url);

    res_future.map(|res| {
        if res.status() != StatusCode::OK {
            return Body::from(format!("Bad status code: {}", res.status()));
        }

        res.into_body()
    })
}

fn router(req: Request<Body>) -> BoxFuture {
    let mut response = Response::new(Body::empty());
    
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("GET on /domain/somedomain.com");
        }

        (&Method::GET, "/domain") => {
            let query = parse(req.uri().query().unwrap()).unwrap();
            let mut domain: String = query["domain"].to_string();

            if domain == "null" {
                *response.body_mut() = Body::from("'domain' is a required query parameter");
            } else {
                domain = domain.replace("\"", "");
                println!("Domain requested: {}", domain);
                let uri = build_builtwith_hyper_uri(domain);

                let res = request(uri).wait();
                let body = res.unwrap();

                *response.body_mut() = body;
            }
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
