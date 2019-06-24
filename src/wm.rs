#![allow(dead_code)]
use std::collections::HashMap;
pub use xlib::{Display, Rect, Window};
use xlib::{Event, EventKind, XResult};

/// Events that may be transmitted from the WM to the layout handler.
pub enum EventNotify {
    ButtonPress(u32),
    ButtonRelease(u32),
    Map,
    Unmap,
    Resize,
}

/// The trait that needs to be implemented by the layout module.
pub trait EventRx {
    // ? init?
    // ? &self isn't necessary but might be helpful(?)
    fn setup(&self, display: &Display, root: &Window);
    fn notify(&self, event: EventNotify, window: &mut Window);
}

/// The main structure of the window manager. It is responsible for connecting
/// to X, handling incoming events, and calling the functions that are extended
/// through the `EventTx` trait.
///
/// This structure is responsible for creating, framing, and mapping any
/// windows that we manage. Any layout-related events, like resizing and
/// moving, is handled by the aforementioned layout component.
pub struct WM<'a, T: EventRx> {
    display: Display,
    root: Window,
    windows: HashMap<u64, Window>,
    layout: &'a T,
}

impl<'a, T: EventRx> WM<'a, T> {
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

        layout.setup(&display, &root);

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
    /// Triggers a call to `EventRx::notify` with `EventNotify::Map`..
    // * Alternative: self.layout.notify(Event, Window)
    fn map_request(&mut self, mut window: Window) {
        self.layout.notify(EventNotify::Map, &mut window);
        self.display.map_window(&window);
        info!("Mapped window {}", window.as_raw());
        self.windows.insert(window.as_raw(), window);
    }

    /// Removes the given window from the list of managed windows and calls
    /// `EventRx::notify` with `EventNotify::Unmap`.
    fn unmap_window(&mut self, window: u64) {
        if self.windows.contains_key(&window) {
            let mut win = self.windows.remove(&window).unwrap();
            self.layout.notify(EventNotify::Unmap, &mut win);
            info!("Unmapped window {}", window);
            drop(win);
        }
    }

    // ? notify_mut and notify?
    fn button_press(&mut self, window: u64, button: u32) {
        if self.windows.contains_key(&window) {
            let mut window = self.windows.get_mut(&window).unwrap();
            self.layout
                .notify(EventNotify::ButtonPress(button), &mut window);
        }
        debug!("Button {} pressed on window {}", button, window);
    }

    /// The main loop that handles all incoming events from X, and calls
    /// the relevant trait function.
    pub fn run(&mut self) {
        loop {
            let event = self.next_event();
            let kind = event.get_kind();
            debug!("Event received: {:?}", kind);

            match kind {
                EventKind::MapRequest(event) => {
                    let window = Window::from_raw(&self.display, event.window);
                    self.map_request(window);
                }
                EventKind::Unmap(event) => {
                    self.unmap_window(event.window);
                }
                EventKind::Configure(event) => {
                    debug!(
                        "Window {} resized to {}x{}",
                        event.window, event.width, event.height
                    );
                }
                EventKind::ButtonPress(event) => {
                    self.button_press(event.subwindow, event.button);
                }
                EventKind::KeyPress(event) => {
                    self.button_press(event.subwindow, event.keycode);
                }
                _ => {
                    warn!("Event ignored");
                }
            }
        }
    }
}
