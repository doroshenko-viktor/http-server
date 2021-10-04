use std::fmt::{Display, Result as FmtResult};

#[derive(Clone, Copy, Debug)] // TODO: elaborate topic of clone/copy
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    NotFound = 404,
    MethodNotSupported = 405,
    InternalServerError = 500,
}

impl StatusCode {
    pub fn reason_phrase(&self) -> &str {
        match &self {
            Self::Ok => "Ok",
            Self::BadRequest => "Bad Request",
            Self::NotFound => "Not Found",
            Self::MethodNotSupported => "Method Not Supported",
            Self::InternalServerError => "Internal Server Error",
        }
    }
}

impl Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", *self as u16)
    }
}
