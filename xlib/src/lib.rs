extern crate x11;

mod display;
mod errors;
mod events;
mod window;

use x11::xlib;
pub(crate) type XDisplay = *mut xlib::Display;
pub(crate) type XEvent = *mut xlib::XEvent;
pub(crate) type XWindow = xlib::Window;

pub use display::Display;
pub use errors::XError;
pub use events::{Event, EventMask, Events};
pub use window::Window;
pub type XResult<T> = std::result::Result<T, XError>;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}
