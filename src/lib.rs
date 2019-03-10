#[macro_use]
extern crate simple_error;
extern crate chrono;
extern crate futures;
extern crate iron;
extern crate percent_encoding;
extern crate router;
extern crate serde;
extern crate serde_json;
extern crate serde_qs;

pub mod config;
mod geocoder;
mod provider;
mod reporter;
mod server;

pub fn start_server(config: Option<config::Server>) -> iron::Listening {
    server::start(config).expect("Failed to start the server")
}
