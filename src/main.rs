mod wm;
use wm::{Rect, WM};

fn main() {
    let mut wm = WM::new().expect("Failed to connect to X!");

    loop {
        match wm.next_event() {
            x @ _ => println!("{:?}", x.kind()),
        }
    }
}
