extern crate x11;

mod display;
mod errors;
mod events;
mod window;

pub use display::Display;
pub use errors::XError;
pub use events::{EventMask, Events};
pub use window::Window;
pub type XResult<T> = std::result::Result<T, XError>;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
