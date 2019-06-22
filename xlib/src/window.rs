use crate::{Display, Rect, XDisplay, XWindow};
use x11::xlib;

#[derive(Debug)]
pub struct Window {
    display: XDisplay,
    bounds: Rect,
    inner: XWindow,
}

impl Window {
    pub fn new(display: &Display, bounds: Rect) -> Self {
        let window = unsafe {
            xlib::XCreateSimpleWindow(
                display.as_raw(),
                display.default_window().inner,
                bounds.x,
                bounds.y,
                bounds.width,
                bounds.height,
                0,        // border width
                0,        // border color
                16777215, // bg color (256^3-1 = white)
            )
        };

        Self {
            display: display.as_raw(),
            bounds,
            inner: window,
        }
    }

    pub fn from_raw(display: &Display, window: XWindow) -> Self {
        Self {
            display: display.as_raw(),
            bounds: Rect::default(),
            inner: window,
        }
    }

    pub fn as_raw(&self) -> XWindow {
        self.inner
    }

    pub fn get_bounds(&self) -> Rect {
        self.bounds
    }

    pub fn move_resize(&mut self, bounds: Rect) {
        self.bounds = bounds;
        unsafe {
            xlib::XMoveResizeWindow(
                self.display,
                self.inner,
                self.bounds.x,
                self.bounds.y,
                bounds.width,
                bounds.height,
            );
        }
    }
}

// impl Drop for Window {
//     fn drop(&mut self) {
//         // Destroys (and unmaps) the window
//         unsafe {
//             xlib::XUnmapWindow(self.display, self.inner);
//         }
//     }
// }
