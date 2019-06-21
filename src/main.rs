#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod server;
mod wm;

use server::Server;
use wm::WM;

const SOCKET: &str = "/tmp/wm-rs.sock";

fn main() {
    pretty_env_logger::init();

    let server = Server::new(SOCKET);
    let mut wm = WM::new().expect("Failed to connect to X!");
    info!("Connected to X");

    wm.run();
}

// * Attach a stream of some sort to the WM
// * This can thus be customized to either send off events as a server,
// * or handle them directly in another Rust module.
// TODO: Make a trait for it.
// TODO: Must implement Send, Receive, and...?
// E.g.:
//  trait EventTX {
//      fn send();
//      fn recv();
//  }
