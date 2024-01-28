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

pub struct Error {
    id: String,
    kind: ErrorKind,
    msg: String,
    trace: Vec<Location<'static>>,
    source: Option<Box<ErrorSource>>,
}


macro_rules! error_kind {
    ( $( $name:ident $status:ident ),* $(,)? ) => {
        #[derive(Clone, Copy, Enum, Serialize, Debug, PartialEq, Eq, Hash)]
        pub enum ErrorKind {$(
            $name,
        )*}

        impl From<ErrorKind> for http::StatusCode {
            fn from(kind: ErrorKind) -> Self {
                use http::StatusCode;
                use ErrorKind::*;
                match kind {$(
                    $name => StatusCode::$status,
                )*}
            }
        }

        impl ErrorKind {
            pub fn iter() -> Iter<'static, ErrorKind> {
                use ErrorKind::*;
                static ALL: &[ErrorKind] = &[$(
                    $name,
                )*];
                ALL.iter()
            }
        }
    };
}

error_kind! {
    NotFound NOT_FOUND,
    BadRequest BAD_REQUEST,
    InvalidCharacter BAD_REQUEST,
    Unauthorized UNAUTHORIZED,
    AccessDenied FORBIDDEN,
    InternalError INTERNAL_SERVER_ERROR,
    UnknownError INTERNAL_SERVER_ERROR,
    TimeoutError INTERNAL_SERVER_ERROR,
    RedirectError INTERNAL_SERVER_ERROR,
}

/// Use in combination with `Result::map_err` in order to map any error
/// into the `Error` struct.
/// 
/// The resulting Error is populated with `kind` set to the result of
/// converting the `E` into `ErrorKind`, which means you *must* implement
/// `From<E> for ErrorKind`.
/// 
/// The `trace` is populated with the location of where `wrap_err` was called.
/// 
/// The `source` is set appropriately.
#[track_caller]
pub fn wrap_err<E>(source: E) -> Error
    where
        E: Into<Box<ErrorSource>> + Send + Sync + 'static,
        ErrorKind: for<'a> From<&'a E>
{
    Error::from_source(ErrorKind::from(&source), source.into(), *Location::caller())
}

/// Import this trait if you want the `wrap_err` function to be more easily
/// invoked directly on the `Result` object.
pub trait WrapErr<T, E> {
    fn wrap_err(self) -> Result<T>;
    fn wrap_err_msg(self, lazy_msg: impl FnOnce(&E) -> String) -> Result<T>;
}

impl <T, E> WrapErr<T, E> for result::Result<T, E>
    where
        E: Into<Box<ErrorSource>> + Send + Sync + 'static,
        ErrorKind: for<'a> From<&'a E>
{
    #[track_caller]
    fn wrap_err(self) -> Result<T> {
        self.wrap_err_msg(|_| String::new())
    }

    #[track_caller]
    fn wrap_err_msg(self, lazy_msg: impl FnOnce(&E) -> String) -> Result<T> {
        match self {
            Err(source) => {
                let kind = ErrorKind::from(&source);
                let msg = lazy_msg(&source);
                let error = Error::from_source_with_msg(
                    kind,
                    msg,
                    source.into(),
                    *Location::caller());
                Err(error)
            },
            Ok(ok) => Ok(ok),
        }
    }
}

/// It is highly suggested to add `.trace()?` calls rather than just `?`
/// so that a stack frame can be automatically recorded. Note that if you
/// call `.context()` the stack frame is automatically recorded, but using
/// that method allows you to augement the `msg` with context information.
/// 
/// Note that the `msg` of the error is updated with the results of calling
/// the `lazy_msg(&err)`, so it is recommended to format the msg so it includes
/// the results of `err.msg()`.
pub trait Trace {
    fn trace(self) -> Self;
    fn context(self, lazy_msg: impl FnOnce(&Error) -> String) -> Self;
}

impl <T> Trace for Result<T> {
    #[track_caller]
    fn trace(mut self) -> Self {
        if let Err(err) = &mut self {
            err.trace.push(*Location::caller());
        }
        self
    }
    #[track_caller]
    fn context(mut self, lazy_msg: impl FnOnce(&Error) -> String) -> Self {
        if let Err(mut err) = self {
            err.trace.push(*Location::caller());
            err.msg = lazy_msg(&err);
            self = Err(err);
        }
        self
    }
}

pub trait OrNotFound<T> {
    fn or_not_found(self, lazy_msg: impl FnOnce() -> String) -> Result<T>;
}

impl <T> OrNotFound<T> for Option<T> {
    #[track_caller]
    fn or_not_found(self, lazy_msg: impl FnOnce() -> String) -> Result<T> {
        match self {
            Some(found) => Ok(found),
            None => Err(Error::new(
                ErrorKind::NotFound,
                lazy_msg(),
                *Location::caller(),
            ))
        }
    }
}

impl From<ErrorKind> for u16 {
    fn from(kind: ErrorKind) -> Self {
        http::StatusCode::from(kind).as_u16()
    }
}

impl From<ErrorKind> for String {
    fn from(kind: ErrorKind) -> String {
        format!("{kind:?}")
    }
}

// TRANSLATE ERRORS:
impl From<&std::io::Error> for ErrorKind  {
    fn from(err: &std::io::Error) -> ErrorKind {
        ErrorKind::UnknownError // TODO
    }
}

impl From<&url::ParseError> for ErrorKind{
    fn from(err: &url::ParseError) -> ErrorKind {
        ErrorKind::UnknownError // TODO
    }
}

impl From<&std::env::VarError> for ErrorKind {
    fn from(err: &std::env::VarError) -> ErrorKind {
        use std::env::VarError::*;
        match err {
            NotPresent => ErrorKind::NotFound,
            NotUnicode(_) => ErrorKind::InvalidCharacter,
        }
    }
}

impl <E, R> From<&aws_smithy_runtime_api::client::result::SdkError<E, R>> for ErrorKind {
    fn from(err: &aws_smithy_runtime_api::client::result::SdkError<E, R>) -> ErrorKind {
        ErrorKind::UnknownError // TODO
    }
}

impl From<& reqwest::Error> for ErrorKind {
    fn from(err: &reqwest::Error) -> ErrorKind {
        use ErrorKind::*;
        if let Some(status) = err.status() {
            let code = status.as_u16();
            match code {
                404 => NotFound,
                400 => BadRequest,
                401 => Unauthorized,
                403 => AccessDenied,
                _ => {
                    match code / 100 {
                        3 => RedirectError,
                        4 => BadRequest,
                        5 => InternalError,
                        _ => UnknownError,
                    }
                }
            }
         } else if err.is_timeout() {
            TimeoutError
        } else if err.is_redirect() {
            RedirectError
        } else {
            UnknownError
        }
    }
}

impl From<&Box<ErrorSource>> for ErrorKind {
    fn from(_: &Box<ErrorSource>) -> ErrorKind {
        // Can't really infer a type of error since this is opaque
        ErrorKind::UnknownError
    }
}

impl Error {
    pub fn new(kind: ErrorKind, msg: String, location: Location<'static>) -> Self {
        Self {
            id: unique_id(),
            kind,
            msg,
            trace: vec![location],
            source: None,
        }
    }

    pub fn from_source(kind: ErrorKind, source: Box<ErrorSource>, location: Location<'static>) -> Self {
        Self::from_source_with_msg(kind, String::new(), source, location)
    }

    pub fn from_source_with_msg(kind: ErrorKind, msg: String, source: Box<ErrorSource>, location: Location<'static>) -> Self {
        Self {
            id: unique_id(),
            kind,
            msg,
            trace: vec![location],
            source: Some(source),
        }
    }

    pub fn add_location(mut self, location: Location<'static>) -> Self {
        self.trace.push(location);
        self
    }

    pub fn with_msg(mut self, msg: String) -> Self {
        self.msg = msg;
        self
    }
    pub fn with_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }

    pub fn trace(&self) -> impl Iterator<Item = &Location<'static>> {
        self.trace.iter()
    }

    pub fn msg(&self) -> Cow<str> {
        if self.msg.is_empty() {
            match &self.source {
                None => Cow::Borrowed("null"),
                Some(src) => Cow::Owned(format!("{src}")),
            }
        } else {
            Cow::Borrowed(&self.msg)
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

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.source {
            None => None,
            Some(src) => Some(src.as_ref()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{:?}: {}", self.kind, self.msg())?;
        for location in self.trace() {
            writeln!(f, "\tat {} line {}", location.file(), location.line())?;
        }
        match self.source {
            None => (),
            Some(ref src) => write!(f, "Caused By: {src}")?,
        }
        Ok(())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self}")
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
        self.kind().into()
    }
    fn as_response(&self) -> Response {
        let kind = self.kind.into();
        let detail = self.msg().into_owned();
        let title = format!("{kind:?}: {detail}");
        let payload = Json(ErrorResponse {
            kind,
            title,
            status: self.kind.into(),
            detail,
            instance: self.id.clone(),
        });
        use ErrorResponseWrapper::*;
        match kind.into() {
            404 => NotFound(payload),
            400 => BadRequest(payload),
            401 => Unauthorized(payload),
            403 => AccessDenied(payload),
            _ => InternalError(payload),
        }.into_response()
    }
}

fn unique_id() -> String {
    let mut rand = [0u8; 15]; // max 24 chars, 120 bits of entropy
    OsRng.fill_bytes(&mut rand);
    base32::encode(base32::Alphabet::Crockford, &rand)
}
