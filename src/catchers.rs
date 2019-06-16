#![allow(non_snake_case)]

use crate::error::Error;
use rocket::http::Status;
use rocket::Catcher;

macro_rules! gen_err_catchers {
    ($($code:expr, $name:ident),+) => {
        $(
            #[catch($code)]
            pub fn $name() -> Error {
                Error::from(Status::raw($code))
            }
        )+

        pub fn All() -> Vec<Catcher> {
            catchers![$($name),*]
        }
    };
}

gen_err_catchers! {
    400, BadRequest,
    401, Unauthorized,
    402, PaymentRequired,
    403, Forbidden,
    404, NotFound,
    405, MethodNotAllowed,
    406, NotAcceptable,
    407, ProxyAuthenticationRequired,
    408, RequestTimeout,
    409, Conflict,
    410, Gone,
    411, LengthRequired,
    412, PreconditionFailed,
    413, PayloadTooLarge,
    414, UriTooLong,
    415, UnsupportedMediaType,
    416, RangeNotSatisfiable,
    417, ExpectationFailed,
    418, ImATeapot,
    421, MisdirectedRequest,
    422, UnprocessableEntity,
    423, Locked,
    424, FailedDependency,
    426, UpgradeRequired,
    428, PreconditionRequired,
    429, TooManyRequests,
    431, RequestHeaderFieldsTooLarge,
    451, UnavailableForLegalReasons,
    500, InternalServerError,
    501, NotImplemented,
    502, BadGateway,
    503, ServiceUnavailable,
    504, GatewayTimeout,
    505, HttpVersionNotSupported,
    506, VariantAlsoNegotiates,
    507, InsufficientStorage,
    508, LoopDetected,
    510, NotExtended,
    511, NetworkAuthenticationRequired
}
