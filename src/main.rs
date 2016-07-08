extern crate iron;
extern crate router;
extern crate rustc_serialize;

mod controllers;
mod models;
mod routes;

use iron::prelude::*;

fn main() {
    let router = routes::router();
    Iron::new(router).http("localhost:9999").unwrap();
}
