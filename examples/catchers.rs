#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_json::Error;

#[catch(500)]
fn json_500() -> Error {
    Error::InternalServerError
}

#[catch(404)]
fn json_404() -> Error {
    Error::NotFound
}

fn main() {
    rocket::ignite().register(catchers![json_500, json_404]).launch();
}
