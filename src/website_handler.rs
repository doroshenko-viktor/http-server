use super::http::{Method, Response, StatusCode};
use super::server::RequestHandler;
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        WebsiteHandler { public_path }
    }

    fn get_file(&self, file_path: &str) -> Option<String> {
        let file_path = if !file_path.starts_with('/') {
            format!("/{}", file_path)
        } else {
            file_path.to_string()
        };
        let path = format!("{}{}", self.public_path, file_path);
        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    let msg = format!("Directory traversal attack attempted");
                    Some(msg)
                }
            }
            Err(_) => {
                let msg = format!("Directory traversal attack attempted");
                Some(msg)
            }
        }
    }
}

impl RequestHandler for WebsiteHandler {
    fn handle_request(&self, request: &crate::http::Request) -> Response {
        match request.method() {
            &Method::GET => match request.path() {
                "/" => {
                    let r = self.get_file("index.html");
                    match r {
                        Some(body) => Response::new(StatusCode::Ok, Some(body)),
                        None => Response::new(
                            StatusCode::InternalServerError,
                            Some(String::from("<h1>serving page not found</h1>")),
                        ),
                    }
                }
                "/test" => Response::new(StatusCode::Ok, self.get_file("test.html")),
                path => match self.get_file(path) {
                    Some(data) => Response::new(StatusCode::Ok, Some(data)),
                    _ => Response::new(
                        StatusCode::NotFound,
                        Some(String::from(format!(
                            "<h1>Not Found</h1><p>{} endpoint not supported</p>",
                            request.path()
                        ))),
                    ),
                },
            },
            &Method::POST => Response::new(
                StatusCode::BadRequest,
                Some(String::from("<h1>POST response</h1>")),
            ),
            method => {
                let msg = format!("<h1>Error</h1><p>METHOD {} NOT SUPPORTED", method);
                Response::new(StatusCode::MethodNotSupported, Some(msg))
            }
        }
    }

    fn handle_bad_request(&self, e: &crate::http::ParseError) -> Response {
        Response::new(crate::http::StatusCode::BadRequest, Some(format!("{}", e)))
    }
}
