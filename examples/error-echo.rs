#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::response::Result;
use rocket_json::catchers;

#[get("/<status_code>")]
fn error<'r>(status_code: u16) -> Result<'r> {
    Err(Status::raw(status_code))
}

fn main() {
    rocket::ignite()
        .register(catchers::All())
        .mount("/", routes![error])
        .launch();
}
