pub use xlib::Rect;
use xlib::{Display, Event, EventMask, Window, XResult};

pub struct WM {
    display: Display,
    root: Window,
}

impl WM {
    pub fn new() -> XResult<Self> {
        // Connect to the default display of X
        let display = Display::connect(None)?;
        // Get the root window of the display
        let root = display.default_window();
        // Register that we want to receive events for structuring concerning  the root window
        display.select_input(&root, EventMask::SubstructureNotifyMask);

        Ok(Self { display, root })
    }

    pub fn create_window(&mut self, bounds: Rect) {
        let win = Window::new(&self.display, bounds);
        // Map the window to the display
        self.display.map_window(&win);
        // Synchronize the display without discarding pending events
        self.display.sync(false);
    }

    pub fn next_event(&mut self) -> Event {
        self.display.next_event()
    }
}
