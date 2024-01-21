use std::error;
use std::panic::Location;
use std::result;
use poem_openapi::ApiResponse;
use poem_openapi::Object;
use poem_openapi::Enum;
use poem::error::ResponseError;
use std::fmt;
use poem_openapi::registry::MetaResponses;
use poem_openapi::registry::Registry;
use serde::Serialize;
use std::slice::Iter;
use std::borrow::Cow;
use poem::Response;
use poem_openapi::payload::Json;
use poem::IntoResponse;
use rand::rngs::OsRng;
use rand::RngCore;

pub type Result<T> = result::Result<T, Error>;
pub type ErrorSource = dyn error::Error + Send + Sync + 'static;

#[derive(Debug)]
pub struct Error {
    pub id: String,
    pub kind: ErrorKind,
    pub msg: String,
    pub trace: Vec<Location<'static>>,
    pub source: ErrorChain,
}

#[derive(Debug)]
pub enum ErrorChain {
    Final,
    Next(Box<Error>),
    Source(Box<ErrorSource>),
}

#[derive(Clone, Copy, Enum, Serialize, Debug, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    NotFound,
    BadRequest,
    Unauthorized,
    AccessDenied,
    InternalError,
}

fn unique_id() -> String {
    let mut rand = [0u8; 15]; // max 24 chars, 120 bits of entropy
    OsRng.fill_bytes(&mut rand);
    base32::encode(base32::Alphabet::Crockford, &rand)
}

#[track_caller]
pub fn err(kind: ErrorKind, msg: String) -> Error {
    Error {
        id: unique_id(),
        kind,
        msg,
        trace: vec![*Location::caller()],
        source: ErrorChain::Final,
    }
}

#[track_caller]
pub fn wrap_err<E: error::Error + Send + Sync + 'static>(source: E) -> Error {
    wrap_err_with_location(Box::new(source), *Location::caller())
}

#[track_caller]
pub fn wrap_boxed_err(source: Box<ErrorSource>) -> Error {
    wrap_err_with_location(source, *Location::caller())
}

#[track_caller]
pub fn wrap_err_with_location(source: Box<ErrorSource>, location: Location<'static>) -> Error {
    Error {
        id: unique_id(),
        kind: ErrorKind::InternalError,
        msg: String::new(),
        trace: vec![location],
        source: ErrorChain::Source(source),
    }
}

pub trait Trace {
    fn trace(self) -> Self;
    fn context(self, add_ctx: impl FnOnce(String) -> String) -> Self;
}

impl Trace for Error {
    #[track_caller]
    fn trace(mut self) -> Self {
        self.trace.push(*Location::caller());
        self
    }
    #[track_caller]
    fn context(mut self, add_ctx: impl FnOnce(String) -> String) -> Self {
        self.trace.push(*Location::caller());
        self.msg = add_ctx(self.msg);
        self
    }
}

impl ErrorKind {
    pub fn as_status_code(&self) -> http::StatusCode {
        use http::StatusCode;
        use ErrorKind::*;
        match self {
            NotFound => StatusCode::NOT_FOUND,
            BadRequest => StatusCode::BAD_REQUEST,
            Unauthorized => StatusCode::UNAUTHORIZED,
            AccessDenied => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    pub fn iter() -> Iter<'static, ErrorKind> {
        use ErrorKind::*;
        static ALL: &[ErrorKind] = &[
            NotFound, BadRequest, Unauthorized, AccessDenied, InternalError
        ];
        ALL.iter()
    }
}

impl From<ErrorKind> for String {
    fn from(kind: ErrorKind) -> String {
        format!("{kind:?}")
    }
}

impl Error {
    fn get_msg(&self) -> Cow<str> {
        let mut err = self;
        loop {
            if !err.msg.is_empty() {
                return Cow::Borrowed(&err.msg);
            }
            match &err.source {
                ErrorChain::Final => {
                    return Cow::Borrowed("null")
                },
                ErrorChain::Next(next_err) => {
                    err = next_err;
                },
                ErrorChain::Source(src) => {
                    return Cow::Owned(format!("{src}"));
                },
            }
        }
    }
}

#[derive(Clone, ApiResponse, Debug, PartialEq, Eq)]
pub enum ErrorResponseWrapper {
    #[oai(status = 404)]
    NotFound(Json<ErrorResponse>),
    #[oai(status = 500)]
    InternalError(Json<ErrorResponse>),
    #[oai(status = 400)]
    BadRequest(Json<ErrorResponse>),
    #[oai(status = 401)]
    Unauthorized(Json<ErrorResponse>),
    #[oai(status = 403)]
    AccessDenied(Json<ErrorResponse>),
}

// implements  https://datatracker.ietf.org/doc/html/rfc7807#section-3.1
#[derive(Clone, Object, Serialize, Debug, PartialEq, Eq, Hash)]
pub struct ErrorResponse {
    #[oai(rename = "type")]
    #[serde(rename = "type")]
    pub kind: ErrorKind,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub instance: String,
}

macro_rules! impl_from {
    ($err_name:ty) => {
        impl From<$err_name> for Error {
            #[track_caller]
            fn from(source: $err_name) -> Self {
                wrap_err_with_location(Box::new(source), *Location::caller())
            }
        }    
    }
}

impl_from!(url::ParseError);
impl_from!(reqwest::Error);
impl_from!(std::env::VarError);
impl_from!(std::io::Error);

//impl_from!(aws_smithy_runtime_api::client::result::SdkError<T, R>);

impl <E, R> From<aws_smithy_runtime_api::client::result::SdkError<E, R>> for Error
    where
        E: error::Error + 'static + Send + Sync,
        R: fmt::Debug + Send + Sync + 'static,
{
    #[track_caller]
    fn from(source: aws_smithy_runtime_api::client::result::SdkError<E, R>) -> Self {
        wrap_err_with_location(Box::new(source), *Location::caller())
    }
}


impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            ErrorChain::Final => None,
            ErrorChain::Next(src) => Some(src.as_ref()),
            ErrorChain::Source(src) => Some(src.as_ref()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.msg.is_empty() {
            writeln!(f, "{:?}", self.kind)?;
        } else {
            writeln!(f, "{:?}: {}", self.kind, self.msg)?;
        }
        for location in &self.trace {
            writeln!(f, "\tat{} line {}", location.file(), location.line())?;
        }
        match self.source {
            ErrorChain::Final => (),
            ErrorChain::Next(ref src) => write!(f, "Caused By: {src}")?,
            ErrorChain::Source(ref src) => write!(f, "Caused By: {src}")?,
        }
        Ok(())
    }
}

impl ApiResponse for Error {
    fn meta() -> MetaResponses {
        ErrorResponseWrapper::meta()
    }
    fn register(registry: &mut Registry) {
        ErrorResponseWrapper::register(registry)
    }
}

impl ResponseError for Error {
    fn status(&self) -> http::StatusCode {
        self.kind.as_status_code()
    }
    fn as_response(&self) -> Response {
        let kind = self.kind.into();
        let detail = self.get_msg().into_owned();
        let title = format!("{kind:?}: {detail}");
        let payload = Json(ErrorResponse {
            kind,
            title,
            status: self.kind.as_status_code().as_u16(),
            detail,
            instance: self.id.clone(),
        });
        use ErrorResponseWrapper::*;
        match kind.as_status_code().as_u16() {
            404 => NotFound(payload),
            400 => BadRequest(payload),
            401 => Unauthorized(payload),
            403 => AccessDenied(payload),
            _ => InternalError(payload),
        }.into_response()
    }
}