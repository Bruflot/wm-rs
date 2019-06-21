use std::collections::HashMap;
use xlib::EventKind;
use xlib::Rect;
use xlib::{Display, Event, Window, XResult};

/// The main structure of the application. It is responsible for opening a
/// connection to X, fetching the root window, registering events, and setting
/// up the socket server.
pub struct WM {
    display: Display,
    root: Window,
    windows: HashMap<u64, Window>,
}

impl WM {
    /// Constructor for the `WM` class.
    /// Opens a connection to X, fetches the root window, and registers
    /// that we want to receive events regarding structuring of the root window
    /// and its children..
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

    /// Fetches the next event from X. This is a blocking function, and thus
    /// waits until an event is received.
    pub fn next_event(&mut self) -> Event {
        self.display.next_event()
    }

    /// Adds the window to the our list of managed windows, and maps the window
    /// to the active display.
    /// Triggers a `MapRequest` event.
    pub fn map_request(&mut self, window: Window) {
        self.display.map_window(&window);
        info!("Mapped window {}", window.as_raw());
        self.windows.insert(window.as_raw(), window);
    }

    /// Resizes the given window. Triggers a `Resize` event if we manage the
    /// window.
    pub fn resize_window(&mut self, window: u64) {
        if self.windows.contains_key(&window) {
            let bounds = Rect::default();
            let win = self.windows.get_mut(&window).unwrap();
            win.move_resize(bounds);
            info!(
                "Resized window {} to {}x{}",
                window, bounds.width, bounds.height
            );
        }
    }

    /// Removes the given window from the list of managed windows and sends a
    /// `Unmap` event. Only triggered if we manage the window.
    ///
    /// The `Drop` trait of the `window` structure will destroy and unmap the
    /// window automatically.
    pub fn unmap_window(&mut self, window: u64) {
        // Only unmap the window if we manage it
        if self.windows.contains_key(&window) {
            self.windows.remove(&window).unwrap();
            // Server::SendEvent(Event::UnmapWindow, window: Window)
            info!("Unmapped window {}", window);
        }
    }

    /// The main loop that handles all incoming events from X, and calls
    /// the relevant class function.
    pub fn run(&mut self) {
        loop {
            let event = self.next_event();
            let kind = event.get_kind();
            info!("Event received: {:?}", kind);

            match kind {
                EventKind::MapRequest(event) => {
                    let window = Window::from_raw(&self.display, event.window);
                    self.map_request(window);
                }
                EventKind::Unmap(event) => {
                    self.unmap_window(event.window);
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
