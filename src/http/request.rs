use super::{method::MethodError, Method, QueryParams};
use std::{
    convert::TryFrom,
    error::Error,
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    str::{self, Utf8Error},
};

#[derive(Debug)]
pub struct Request<'buf> {
    path: &'buf str,
    query_params: Option<QueryParams<'buf>>,
    method: Method,
}

impl<'buf> Request<'buf> {
    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn query_params(&self) -> Option<&QueryParams<'buf>> {
        self.query_params.as_ref()
    }
}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
    type Error = ParseError;

    //* Example request to parse:
    //* GET /search?name=abc&name1=abc1 HTTP/1.1
    fn try_from(value: &'buf [u8]) -> Result<Self, Self::Error> {
        // match str::from_utf8(value) {
        //     Ok(res) => {}
        //     Err(_) => return Err(ParseError::InvalidEncoding),
        // } This is the same as ↓
        // let request = str::from_utf8(value).or(Err(ParseError::InvalidEncoding))?; It may be simplified if we implement From trait for Utf8Error ↓
        let request = str::from_utf8(value)?;

        // match get_next_word(request) {
        //     Some((method, rest)) => {}
        //     None => ParseError::InvalidRequest,
        // } This could be simplified ↓

        // here it is possible to use `request` as a result var name. it will not modify existing `request` but will shadow it;
        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        let method: Method = method.parse()?;

        let (path, query_params) = match path.find('?') {
            Some(i) => (&path[..i], Some(QueryParams::from(&path[i + 1..]))),
            None => (path, None),
        };

        Ok(Request {
            path,
            query_params,
            method,
        })
    }
}

fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (index, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..index], &request[index + 1..]));
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod(String),
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod(_) => "Invalid Method",
        }
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Error for ParseError {}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(e: MethodError) -> Self {
        Self::InvalidMethod(e.0)
    }
}
