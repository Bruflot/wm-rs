use crate::display::Display;
use crate::{Rect, XResult};
use x11::xlib;

type XDisplay = *mut xlib::Display;
type XWindow = xlib::Window;

#[derive(Debug)]
pub struct Window {
    pub(crate) display: XDisplay,
    pub(crate) inner: XWindow,
}

impl Window {
    pub fn new(display: &Display, bounds: Rect) -> Self {
        let window = unsafe {
            xlib::XCreateSimpleWindow(
                display.inner,
                display.default_window().inner,
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                0,        // border
                0,        // border color
                16777215, // bg color (256^3-1 = white)
            )
        };

        Self {
            display: display.inner,
            inner: window,
        }
    }

    pub fn move_resize(&self, bounds: Rect) {
        unsafe {
            xlib::XMoveResizeWindow(
                self.display,
                self.inner,
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
            );
        }
    }
}
