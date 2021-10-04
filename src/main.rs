#![allow(dead_code)]
mod http;
mod server;
mod website_handler;

use server::Server;
use std::env;
use website_handler::WebsiteHandler;

const DEFAULT_PUBLIC_PATH: &str = "./public";
const DEFAULT_HOST_ADDRESS: &str = "127.0.0.1:8080";

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path is: {}", public_path);
    let host = env::var("HOST_ADDRESS").unwrap_or(DEFAULT_HOST_ADDRESS.to_string());

    let server = Server::new(host);
    let website_handler = WebsiteHandler::new(public_path);
    server.run(website_handler);
}
