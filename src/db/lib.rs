extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use self::dotenv::dotenv;
use std::env;

fn get_connection_url() -> String {
    // format "postgres://postgres:password@localhost/domains"
    dotenv().ok();

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn connect() -> PgConnection {
    let connection_url = get_connection_url();
    
    PgConnection::establish(&connection_url)
        .expect(&format!("Error connecting to {}", connection_url))
}