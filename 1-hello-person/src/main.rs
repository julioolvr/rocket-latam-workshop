#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use rocket::http::RawStr;

// FIXME: Declare the following routes:
//
//   * `simple_hello`
//
//      GET '/<name>' => "Hello, <name>!"
//
//   * `good_aged_hello`
//
//      GET '/<name>/<age>' => "Hello, <age> year old <name>."
//
//      where 0 < age <= 122
//
//   * `bad_aged_hello`
//
//      GET '/<name>/<age>' => "'<age>' is a funky age, <name>."
//
//      where 0 < age <= usize::max_value()
//

struct ValidAge(usize);

impl<'r> rocket::request::FromParam<'r> for ValidAge {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        match usize::from_param(param) {
            Ok(age) if age <= 122 => Ok(ValidAge(age)),
            _ => Err(RawStr::from_str("Invalid age"))
        }
    }
}

#[get("/<name>")]
fn simple_hello(name: String) -> String {
    format!("Hello, {}!", name)
}

#[get("/<name>/<age>")]
fn good_aged_hello(name: String, age: ValidAge) -> String {
    format!("Hello, {} year old {}.", age.0, name)
}

#[get("/<name>/<age>", rank = 2)]
fn bad_aged_hello(name: String, age: usize) -> String {
    format!("'{}' is a funky age, {}.", age, name)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![simple_hello, good_aged_hello, bad_aged_hello])
        .launch();
}
