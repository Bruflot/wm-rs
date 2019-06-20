pub use xlib::Events;
pub use xlib::Rect;
use xlib::{Display, Event, EventMask, Window, XResult};

pub struct WM {
    display: Display,
    windows: Vec<Window>
}

impl WM {
    pub fn new() -> XResult<Self> {
        let display = Display::connect(None)?;
        let root = display.default_window();
        display.select_input(
            &root,
            EventMask::SubstructureNotifyMask | EventMask::SubstructureRedirectMask,
        );

        Ok(Self { 
            display, 
            windows: Vec::new() 
        })
    }

    pub fn map_request(&self, window: &Window) {
        self.display.map_window(&window);
    }

    pub fn next_event(&mut self) -> Event {
        self.display.next_event()
    }

    pub fn run(&mut self) {
        loop {
            let event = self.next_event();
            let kind = event.get_kind();
            info!("Event received: {:?}", kind);

            match kind {
                Events::MapRequest => {
                    let event_window = event.get_map_event().window;
                    let window = Window::from_raw(&self.display, event_window);
                    self.map_request(&window);
                    info!("Mapped window {}", window.as_raw());
                    self.windows.push(window);
                },
                _ => warn!("Event ignored"),
            }
        }
    }
}
