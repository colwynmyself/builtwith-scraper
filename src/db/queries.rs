extern crate diesel;

use diesel::pg::PgConnection;

use db::models::{Request};
use db::schema::request;
use diesel::prelude::*;

pub fn read_requests(conn: &PgConnection) {
    let results = request::table
        .limit(1)
        .load::<Request>(&conn)
        .expect("Error loading requests");

    println!("Displaying {} requests", results.len());
    for r in results {
        println!("{}", r.domain);
        println!("----------\n");
    }
}