#![warn(unused_extern_crates)]
#[macro_use]
extern crate derive_more;
extern crate mio;
#[macro_use]
extern crate envconfig_derive;
extern crate envconfig;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate slab;
#[macro_use]
extern crate serde;

use core::{Server, ServerConfig};

mod core;
mod models;
mod requests;
mod store;
mod utils;

fn main() {
    ::std::env::set_var("RUST_LOG", "debug");
    env_logger::init();
    if let Err(err) = ServerConfig::new().and_then(Server::run) {
        error!("Failed to start the core: {}", err);
    }
}
