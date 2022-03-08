#[macro_use] extern crate rocket;
mod commands;
mod server;
mod setup;
mod ipfs;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "setup" {
            setup::setup();
        }
    }

    println!("Starting IPFS");
    ipfs::start_daemon();

    println!("adding bootstap nodes");
    ipfs::add_bootstrap_peers();

    println!("Starting server...");
    // Start the server
    server::rocket();
}
