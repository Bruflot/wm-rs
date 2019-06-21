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
pub use events::{Event, EventKind};
pub use window::Window;
pub type XResult<T> = std::result::Result<T, XError>;

pub const KEY_PRESS_MASK: i64 = 0x0000_0001;
pub const KEY_PRESS_RELEASE: i64 = 0x0000_0002;
pub const BUTTON_PRESS_MASK: i64 = 0x0000_0004;
pub const BUTTON_RELEASE_MASK: i64 = 0x0000_0008;
pub const STRUCTURE_NOTIFY_MASK: i64 = 0x0002_0000;
pub const SUBSTRUCTURE_NOTIFY_MASK: i64 = 0x0008_0000;
pub const SUBSTRUCTURE_REDIRECT_MASK: i64 = 0x0010_0000;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for Rect {
    fn default() -> Self {
        Rect {
            x: 0,
            y: 0,
            width: 250,
            height: 250,
        }
    }
}
