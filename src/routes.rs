extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;
use controllers::*;

pub fn router() -> Router {
    let mut router = Router::new();
    router.get("/", Index::home);
    router.get("/tasks", Tasks::list);

    router
}
