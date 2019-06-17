pub use xlib::Rect;
use xlib::{Display, Window, XResult};

pub struct WM {
    display: Display,
    root: Window,
}

impl WM {
    pub fn new() -> XResult<Self> {
        let display = Display::connect::<&str>(None)?;
        let root = display.default_window();
        Ok(Self { display, root })
    }

    pub fn create_window(&mut self, bounds: Rect) {
        let win = Window::new(&self.display, bounds);
        self.display.map_window(&win);
        self.display.sync(false);
    }

    pub fn next_event(&mut self) {
        // self.display.
    }
}
