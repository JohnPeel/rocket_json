use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Error(pub Status);

macro_rules! ctrs {
    ($($name:ident),+) => {
        $(
            #[allow(non_upper_case_globals)]
            pub const $name: Error = Error(Status::$name);
         )+
    }
}
/// A specialized [`Status`](`rocket::http::Status`) struct that responds json errors.
///
/// This struct implements a [`Responder`](`rocket::response::Responder`) that will return json.
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
impl Error {
    ctrs! {
        BadRequest,
        Unauthorized,
        PaymentRequired,
        Forbidden,
        NotFound,
        MethodNotAllowed,
        NotAcceptable,
        ProxyAuthenticationRequired,
        RequestTimeout,
        Conflict,
        Gone,
        LengthRequired,
        PreconditionFailed,
        PayloadTooLarge,
        UriTooLong,
        UnsupportedMediaType,
        RangeNotSatisfiable,
        ExpectationFailed,
        ImATeapot,
        MisdirectedRequest,
        UnprocessableEntity,
        Locked,
        FailedDependency,
        UpgradeRequired,
        PreconditionRequired,
        TooManyRequests,
        RequestHeaderFieldsTooLarge,
        UnavailableForLegalReasons,
        InternalServerError,
        NotImplemented,
        BadGateway,
        ServiceUnavailable,
        GatewayTimeout,
        HttpVersionNotSupported,
        VariantAlsoNegotiates,
        InsufficientStorage,
        LoopDetected,
        NotExtended,
        NetworkAuthenticationRequired
    }
}

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Error(status)
    }
}

impl AsRef<Status> for Error {
    fn as_ref(&self) -> &Status {
        &self.0
    }
}

impl AsMut<Status> for Error {
    fn as_mut(&mut self) -> &mut Status {
        &mut self.0
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let error = json!({
            "status": self.0.code,
            "reason": self.0.reason
        });

        Response::build_from(error.respond_to(req)?)
            .status(self.0)
            .ok()
    }
}
