#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use(catch, catchers)]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod catchers;
pub mod error;
pub mod result;

#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use result::Result;
