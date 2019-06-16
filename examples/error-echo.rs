#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket_json::{Error, Result};

#[get("/<status_code>")]
fn error(status_code: u16) -> Result<()> {
    if let Some(status) = Status::from_code(status_code) {
        return Err(Error(status));
    }
    Ok(())
}

fn main() {
    rocket::ignite().mount("/", routes![error]).launch();
}
