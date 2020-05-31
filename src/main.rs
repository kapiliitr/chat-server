#[macro_use]
extern crate derive_more;
extern crate mio;
extern crate uuid;
#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate httparse;
extern crate slab;
#[macro_use]
extern crate serde;
extern crate serde_json;

use server::{Server, ServerConfig};

mod models;
mod requests;
mod server;
mod store;
mod utils;

fn main() {
    ::std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    if let Err(err) = ServerConfig::new().and_then(|config| Server::run(config)) {
        error!("Failed to start the server: {}", err);
    }
}
