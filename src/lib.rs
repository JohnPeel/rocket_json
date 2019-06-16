
#[macro_use]
extern crate rocket_contrib;

mod error;

pub use error::Error;
use std::result;

pub type Result<R> = result::Result<R, Error>;
