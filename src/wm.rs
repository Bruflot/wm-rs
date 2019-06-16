pub use xlib::Rect;
use xlib::{Display, Window};

pub struct WM {
    display: Display,
    root: Window,
}

impl WM {
    pub fn new() -> xlib::Result<Self> {
        let display = Display::connect::<&str>(None)?;
        let root = display.default_window();
        Ok(Self { display, root })
    }

    pub fn create_window(&mut self, bounds: Rect) {
        let win = Window::new(&self.display, bounds);
        self.display.map_window(&win);
        self.display.sync(false);
    }

    pub fn scale_window(&self, window: Window, bounds: Rect) {
        let width = self.display.get_width() / 2;
        window.move_resize(bounds);
        self.display.sync(false);
    }
}
