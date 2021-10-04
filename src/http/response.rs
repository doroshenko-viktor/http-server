use std::{
    fmt::{self, Display, Result as FmtResult},
    io::{Result as IOResult, Write},
};

use super::StatusCode;

#[derive(Debug)]
pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }

    pub fn send(&self, stream: &mut impl Write) -> IOResult<()> {
        let body = match &self.body {
            Some(x) => x,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> FmtResult {
        let body = match &self.body {
            Some(x) => x,
            None => "",
        };

        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
