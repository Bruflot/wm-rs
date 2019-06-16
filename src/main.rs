mod wm;
use std::thread;
use wm::{Rect, WM};

fn main() {
    let mut wm = WM::new().expect("Failed to connect to X!");
    wm.create_window(Rect {
        x: 0,
        y: 0,
        width: 200,
        height: 200,
    });

    loop {}
}
