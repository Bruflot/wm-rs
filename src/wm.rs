#![allow(dead_code)]
use std::collections::HashMap;
use xlib::{Display, Event, EventKind, XResult};
pub use xlib::{Rect, Window};

/// Events that may be transmitted from the WM to the layout module
pub enum EventNotify {
    Map,
    Unmap,
    Resize,
}

// &self isn't necessary but might be helpful(?)
/// The trait that needs to be implemented by the layout module.
pub trait EventTx {
    fn notify(&self, vent: EventNotify, window: &mut Window);
}

/// The main structure of the window manager. It is responsible for connecting
/// to X, handling incoming events, and calling the functions that are extended
/// through the `EventTx` trait.
/// 
/// This structure is responsible for creating, framing, and mapping any
/// windows that we manage. Any layout-related events, like resizing and
/// moving, is handled by the aforementioned layout component.
pub struct WM<'a, T: EventTx> {
    display: Display,
    root: Window,
    windows: HashMap<u64, Window>,
    layout: &'a T,
}

impl<'a, T: EventTx> WM<'a, T> {
    /// Constructor for the `WM` class.
    /// Creates a new instance of an application. It opens a connection to X,
    /// fetches the root window, and registers that we want to receive events 
    /// regarding structuring of the root window and its children.
    pub fn new(layout: &'a T) -> XResult<Self> {
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
            layout,
        })
    }

    /// Fetches the next event from X. This is a blocking function, and thus
    /// waits until an event is received.
    fn next_event(&mut self) -> Event {
        self.display.next_event()
    }

    /// Adds the window to the our list of managed windows, and maps the window
    /// to the active display.
    /// Triggers a `MapRequest` event.
    // * Alternative: self.layout.notify(Event, Window)
    fn map_request(&mut self, mut window: Window) {
        // self.layout.map(&window);
        self.layout.notify(EventNotify::Map, &mut window);
        self.display.map_window(&window);
        info!("Mapped window {}", window.as_raw());
        self.windows.insert(window.as_raw(), window);
    }

    /// Removes the given window from the list of managed windows and sends a
    /// `Unmap` event. Only triggered if we manage the window.
    ///
    /// The `Drop` trait of the `Window` structure will destroy and unmap the
    /// window automatically.
    fn unmap_window(&mut self, window: u64) {
        if self.windows.contains_key(&window) {
            let mut win = self.windows.remove(&window).unwrap();
            self.layout.notify(EventNotify::Unmap, &mut win);
            info!("Unmapped window {}", window);
        }
    }

    /// Resizes the given window. Triggers a `Resize` event if we manage the
    /// window.
    fn resize_window(&mut self, window: u64) {
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
