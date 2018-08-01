extern crate config;
extern crate hyper;
extern crate hyper_tls;

use std::io::{self, Write};

use hyper::Client;
use hyper::rt::{self, Future, Stream};
use hyper_tls::HttpsConnector;

fn main() {
    let config: config::Config = get_config();
    let builtwith_key: String = config.get_str("builtwith_key").unwrap();
    let url: String = format!("https://api.builtwith.com/free1/api.json?KEY={key}&LOOKUP={url}", key = builtwith_key, url = "colwyn.me");
    let url_parsed: hyper::Uri = url.parse().unwrap();
    
    rt::run(request(url_parsed));
}

fn get_config() -> config::Config {
    let mut config = config::Config::default();

    config
        .merge(config::File::with_name("Config")).unwrap()
        .merge(config::Environment::with_prefix("APP")).unwrap();
    
    return config;
}

fn request(url: hyper::Uri) -> impl Future<Item=(), Error=()> {
    let https = HttpsConnector::new(4).expect("TLS initialization failed");
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    return client
        .get(url)
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        .map(|_| {
            println!("Done!");
        })
        .map_err(|err| {
            eprintln!("Error: {}", err);
        });
}