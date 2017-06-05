extern crate rustc_serialize;
#[macro_use]
extern crate nickel;

use nickel::{JsonBody, HttpRouter, Nickel};

#[derive(RustcDecodable, RustcEncodable)]
struct Person {
    firstname: String,
    lastname: String,
}

fn main() {
    let mut server = Nickel::new();

    server.post("/people",
                middleware! {|request, response|
        let person = request.json_as::<Person>().unwrap();
        format!("Hello, {} {}", person.firstname, person.lastname)
    });

    server.listen("127.0.0.1:6767");
}
