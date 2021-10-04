use request::ParseError;

use crate::http::{request, Request, Response, StatusCode};
use std::convert::TryFrom;
use std::{
    io::{Error, Read},
    net::{TcpListener, TcpStream},
};

pub trait RequestHandler {
    fn handle_request(&self, request: &Request) -> Response;
    fn handle_bad_request(&self, e: &ParseError) -> Response {
        Response::new(StatusCode::BadRequest, Some(format!("{}", e)))
    }
}

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Self { address }
    }

    pub fn run(self, handler: impl RequestHandler) {
        println!("Listening on: {}", self.address);

        let listener = TcpListener::bind(&self.address).unwrap();
        loop {
            match listener.accept() {
                Ok((stream, _)) => handle_tcp_connection(stream, &handler),
                Err(e) => handle_tcp_connection_error(e),
            }
        }

        fn handle_tcp_connection(mut stream: TcpStream, handler: &impl RequestHandler) {
            fn handle_read_request_error(e: Error) {
                println!("Failed to read from connection: {}", e)
            }

            fn read_request_body(
                mut stream: TcpStream,
                buf: [u8; 1024],
                handler: &impl RequestHandler,
            ) {
                println!("Received a request: {}", String::from_utf8_lossy(&buf));
                let response = match Request::try_from(&buf[..]) {
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e),
                };
                let result = response.send(&mut stream);
                let _ = dbg!("Handle result: {}", result);
            }

            let mut buf = [0u8; 1024];
            match stream.read(&mut buf) {
                Ok(_) => read_request_body(stream, buf, handler),
                Err(e) => handle_read_request_error(e),
            }
        }

        fn handle_tcp_connection_error(e: Error) {
            println!("Failed to establish a connection: {}", e);
        }
    }
}
