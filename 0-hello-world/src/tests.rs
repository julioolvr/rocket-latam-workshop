use super::*;
use rocket::{local::Client, http::Status};

#[test]
fn hello_world() {
    let rocket = rocket::ignite().mount("/", routes![index]);
    let client = Client::new(rocket).unwrap();

    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_string(), Some("Hello, world!".into()));
}

#[test]
fn test_head() {
    let rocket = rocket::ignite().mount("/", routes![index]);
    let client = Client::new(rocket).unwrap();

    let mut response = client.head("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.body_bytes(), Some(vec![]));
}
