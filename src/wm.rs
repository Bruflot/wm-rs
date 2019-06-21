use std::collections::HashMap;
pub use xlib::EventKind;
pub use xlib::Rect;
use xlib::{Display, Event, Window, XResult};

pub struct WM {
    display: Display,
    root: Window,
    windows: HashMap<u64, Window>,
}

impl WM {
    pub fn new() -> XResult<Self> {
        let display = Display::connect(None)?;
        let root = display.default_window();
        display.select_input(
            &root,
            xlib::SUBSTRUCTURE_NOTIFY_MASK
                | xlib::SUBSTRUCTURE_REDIRECT_MASK
                | xlib::STRUCTURE_NOTIFY_MASK,
        );

        Ok(Self {
            display,
            root,
            windows: HashMap::new(),
        })
    }

    pub fn map_request(&self, window: &Window) {
        self.display.map_window(&window);
    }

    pub fn next_event(&mut self) -> Event {
        self.display.next_event()
    }

    pub fn resize_window(&mut self, window: u64, bounds: Rect) {
        let win = self.windows.get_mut(&window).unwrap();
        win.move_resize(bounds);
    }

    pub fn run(&mut self) {
        loop {
            let event = self.next_event();
            let kind = event.get_kind();
            info!("Event received: {:?}", kind);

            match kind {
                EventKind::MapRequest(event) => {
                    let window = Window::from_raw(&self.display, event.window);
                    self.map_request(&window);
                    self.windows.insert(window.as_raw(), window);
                    info!("Mapped window {}", event.window);
                }
                EventKind::Unmap(event) => {
                    if self.windows.contains_key(&event.window) {
                        self.windows.remove(&event.window);
                    }
                    info!("Unmapped window {}", event.window);
                }
                EventKind::Configure(event) => {
                    info!(
                        "Window {} resized to {}x{}",
                        event.window, event.width, event.height
                    );
                }
                _ => {
                    warn!("Event ignored");
                }
            }
        }
    }
}
