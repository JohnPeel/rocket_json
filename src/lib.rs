#[macro_use]
extern crate rocket_contrib;

pub mod error;
pub mod result;

pub use error::Error;
pub use result::Result;
