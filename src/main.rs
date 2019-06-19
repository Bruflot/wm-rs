#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod wm;
use wm::{Events, Rect, WM};

fn main() {
    pretty_env_logger::init();

    let mut wm = WM::new().expect("Failed to connect to X!");
    info!("Connected to X");

    wm.run();
}
