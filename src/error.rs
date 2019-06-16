use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use rocket_contrib::json::JsonValue;
use serde_json::Value;
use std::ops::{Deref, DerefMut};

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
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub code: u16,
    pub reason: &'static str,
    data: Option<JsonValue>,
}

macro_rules! ctrs {
    ($($code:expr, $code_str:expr, $name:ident => $reason:expr),+) => {
        $(
            #[doc="[`Error`] with code <b>"]
            #[doc=$code_str]
            #[doc="</b> and reason <i>"]
            #[doc=$reason]
            #[doc="</i>."]
            #[allow(non_upper_case_globals)]
            pub const $name: Error = Error { code: $code, reason: $reason, data: None };
        )+
    };
}

impl Error {
    #[inline(always)]
    pub fn new(code: u16, reason: &'static str) -> Self {
        Error {
            code,
            reason,
            data: None,
        }
    }

    pub fn extend(self, data: JsonValue) -> Self {
        Error {
            code: self.code,
            reason: self.reason,
            data: Some(data),
        }
    }

    ctrs! {
        300, "300", MultipleChoices => "Multiple Choices",
        301, "301", MovedPermanently => "Moved Permanently",
        302, "302", Found => "Found",
        303, "303", SeeOther => "See Other",
        304, "304", NotModified => "Not Modified",
        305, "305", UseProxy => "Use Proxy",
        307, "307", TemporaryRedirect => "Temporary Redirect",
        308, "308", PermanentRedirect => "Permanent Redirect",
        400, "400", BadRequest => "Bad Request",
        401, "401", Unauthorized => "Unauthorized",
        402, "402", PaymentRequired => "Payment Required",
        403, "403", Forbidden => "Forbidden",
        404, "404", NotFound => "Not Found",
        405, "405", MethodNotAllowed => "Method Not Allowed",
        406, "406", NotAcceptable => "Not Acceptable",
        407, "407", ProxyAuthenticationRequired => "Proxy Authentication Required",
        408, "408", RequestTimeout => "Request Timeout",
        409, "409", Conflict => "Conflict",
        410, "410", Gone => "Gone",
        411, "411", LengthRequired => "Length Required",
        412, "412", PreconditionFailed => "Precondition Failed",
        413, "413", PayloadTooLarge => "Payload Too Large",
        414, "414", UriTooLong => "URI Too Long",
        415, "415", UnsupportedMediaType => "Unsupported Media Type",
        416, "416", RangeNotSatisfiable => "Range Not Satisfiable",
        417, "417", ExpectationFailed => "Expectation Failed",
        418, "418", ImATeapot => "I'm a teapot",
        421, "421", MisdirectedRequest => "Misdirected Request",
        422, "422", UnprocessableEntity => "Unprocessable Entity",
        423, "423", Locked => "Locked",
        424, "424", FailedDependency => "Failed Dependency",
        426, "426", UpgradeRequired => "Upgrade Required",
        428, "428", PreconditionRequired => "Precondition Required",
        429, "429", TooManyRequests => "Too Many Requests",
        431, "431", RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
        451, "451", UnavailableForLegalReasons => "Unavailable For Legal Reasons",
        500, "500", InternalServerError => "Internal Server Error",
        501, "501", NotImplemented => "Not Implemented",
        502, "502", BadGateway => "Bad Gateway",
        503, "503", ServiceUnavailable => "Service Unavailable",
        504, "504", GatewayTimeout => "Gateway Timeout",
        505, "505", HttpVersionNotSupported => "HTTP Version Not Supported",
        506, "506", VariantAlsoNegotiates => "Variant Also Negotiates",
        507, "507", InsufficientStorage => "Insufficient Storage",
        508, "508", LoopDetected => "Loop Detected",
        510, "510", NotExtended => "Not Extended",
        511, "511", NetworkAuthenticationRequired => "Network Authentication Required"
    }
}

impl Into<Status> for Error {
    fn into(self) -> Status {
        Status::new(self.code, self.reason)
    }
}

impl From<Status> for Error {
    fn from(status: Status) -> Self {
        Error::new(status.code, status.reason)
    }
}

fn merge(lhs: &mut Value, rhs: &Value) {
    match (lhs, rhs) {
        (&mut Value::Object(ref mut lhs), &Value::Object(ref rhs)) => {
            for (k, v) in rhs {
                merge(lhs.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (&mut Value::Array(ref mut lhs), &Value::Array(ref rhs)) => {
            lhs.extend(rhs.clone());
        }
        (lhs, rhs) => {
            *lhs = rhs.clone();
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, req: &Request) -> Result<Response<'r>, Status> {
        let mut error = json!({
            "status": self.code,
            "reason": self.reason
        });

        if let Some(data) = &self.data {
            merge(error.deref_mut(), data.deref());
        }

        Response::build_from(error.respond_to(req)?)
            .status(self.into())
            .ok()
    }
}
