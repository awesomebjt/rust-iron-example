extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use rustc_serialize::json;
use models::Task;

pub struct Tasks;
pub struct Index;

impl Tasks {
    pub fn list(_: &mut Request) -> IronResult<Response> {
        let task = Task {
            name: String::from("Bill"),
            task: String::from("Do what you want")
        };
        let encoded = json::encode(&task).unwrap();
        Ok(Response::with((status::Ok, encoded)))
    }
}

impl Index {
    pub fn home(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Home page.")))
    }
}
