extern crate iron;

use iron::prelude::*;
use iron::mime::Mime;

fn main() {
    Iron::new(hello_world).http("localhost:3000").unwrap();
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    let content_type = "text/html; charset=utf-8".parse::<Mime>().unwrap();
    let body = r#"
    <title>GCD Calculator</title>
    <form action="/gcd" method="post">
      <input type="text" name="n"/>
      <input type="text" name="n"/>
      <button type="submit">Compute GCD</button>
    </form>
    "#;


    Ok(Response::with((content_type, iron::status::Ok, body)))
}
