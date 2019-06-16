#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

use rocket::http::Status;
use rocket_json::{Error, Result};

#[get("/<status_code>")]
fn error(status_code: u16) -> Result<()> {
    let error = Error::from(Status::raw(status_code));
    Err(error.extend(json!({"reason": "Hello, World!"})))
}

fn main() {
    rocket::ignite().mount("/", routes![error]).launch();
}
