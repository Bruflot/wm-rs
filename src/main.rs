#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod layout;
mod server;
mod wm;

use layout::EventHandler;
use wm::WM;

fn main() {
    pretty_env_logger::init();

    let server = EventHandler;
    let mut wm = WM::new(&server).expect("Failed to connect to X!");
    info!("Connected to X");

    wm.run();
}
