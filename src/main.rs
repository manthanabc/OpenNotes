#[macro_use] extern crate rocket;
mod commands;
mod server;
mod setup;
mod startipfs;

fn main() {
    if let Some(arg) = std::env::args().nth(1) {
        if arg == "setup" {
            setup::setup();
        }
    }

    // Start the server
    server::rocket();
}
