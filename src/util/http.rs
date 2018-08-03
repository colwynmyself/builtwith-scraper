extern crate config;
extern crate hyper;
extern crate hyper_tls;

use hyper::rt::Future;
use hyper::Client;
use hyper::{Body, StatusCode};

pub fn build_builtwith_hyper_uri(domain: String) -> hyper::Uri {
    let mut config = config::Config::default();

    config
        .merge(config::File::with_name("Config")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();

    let builtwith_key: String = config.get_str("builtwith_key").unwrap();
    let url: String = format!("https://api.builtwith.com/free1/api.json?KEY={key}&LOOKUP={url}", key = builtwith_key, url = domain);

    url.parse().unwrap()
}

pub fn request(url: hyper::Uri) -> impl Future<Item=Body, Error=hyper::Error> {
    let https = hyper_tls::HttpsConnector::new(4).expect("TLS initialization failed");
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