use crate::error;
use std::result;

/// A specialized [`Result`](`std::result::Result`) type that uses [`rocket_json::Error`](`error::Error`).
///
/// This typedef is generally used to avoid writing out [`rocket_json::Error`](`error::Error`) directly and
/// is otherwise a direct mapping to [`Result`](`std::result::Result`).
///
/// # Examples
///
/// ```
/// # #[macro_use] extern crate rocket_contrib;
/// # fn main() {
/// use rocket_contrib::json::JsonValue;
/// use rocket_json::{Error, Result};
///
/// fn example(error: bool) -> Result<JsonValue> {
///     if error {
///         // Rocket will return status code 503:
///         // { "reason": "Service Unavailable", "status": 503 }
///         return Err(Error::ServiceUnavailable);
///     }
///     // Rocket will return status code 200:
///     // { "example": 42 }
///     Ok(json!({ "example": 42 }))
/// }
/// # }
pub type Result<R> = result::Result<R, error::Error>;
