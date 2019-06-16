#[macro_use]
extern crate rocket_contrib;

pub mod error;
pub mod result;

#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use result::Result;
