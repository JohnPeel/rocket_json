use rocket::http::Status;
use rocket::request::Request;
use rocket::response::{Responder, Response};
use std::ops::Deref;

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

impl Deref for Error {
    type Target = Status;

    fn deref(&self) -> &Self::Target {
        &self.0
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
            "status": self.code,
            "reason": self.reason
        });

        Response::build_from(error.respond_to(req)?)
            .status(*self)
            .ok()
    }
}
