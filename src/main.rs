#[macro_use]
extern crate router;
extern crate iron;
extern crate urlencoded;

use iron::prelude::*;
use iron::status;
use urlencoded::UrlEncodedBody;

#[allow(unused_variables)]
fn index(req: &mut Request) -> IronResult<Response> {
    let payload = "this page";
    Ok(Response::with((status::Ok, payload)))
}

fn ip_handler(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, req.remote_addr.to_string())))
}

fn post_handler(req: &mut Request) -> IronResult<Response> {
    let payload = match req.get_ref::<UrlEncodedBody>() {
        Ok(ref hashmap) => format!("{:?}", hashmap),
        Err(ref e) => format!("{:?}", e),
    };
    Ok(Response::with((status::Ok, payload)))
}

fn main() {
    let router = router!(get  "/"       => index,
                         get  "/ip"     => ip_handler,
                         post "/post"   => post_handler);
    Iron::new(router).http("localhost:3000").unwrap();
}
