extern crate libc;
extern crate x11;

use crate::XEvent;
use x11::xlib;

// give them better names? CreateNotify instead of CreateWindow?
#[derive(Debug, PartialEq)]
pub enum EventKind {
    Any(xlib::XAnyEvent),
    Button(xlib::XButtonEvent),
    Circulate(xlib::XCirculateEvent),
    CirculateRequest(xlib::XCirculateRequestEvent),
    ClientMessage(xlib::XClientMessageEvent),
    Colormap(xlib::XColormapEvent),
    Configure(xlib::XConfigureEvent),
    ConfigureRequest(xlib::XConfigureRequestEvent),
    CreateWindow(xlib::XCreateWindowEvent),
    Crossing(xlib::XCrossingEvent),
    DestroyWindow(xlib::XDestroyWindowEvent),
    Error(xlib::XErrorEvent),
    Expose(xlib::XExposeEvent),
    FocusChange(xlib::XFocusChangeEvent),
    GenericEventCookie(xlib::XGenericEventCookie),
    GraphicsExpose(xlib::XGraphicsExposeEvent),
    Gravity(xlib::XGravityEvent),
    Key(xlib::XKeyEvent),
    Keymap(xlib::XKeymapEvent),
    Map(xlib::XMapEvent),
    Mapping(xlib::XMappingEvent),
    MapRequest(xlib::XMapRequestEvent),
    Motion(xlib::XMotionEvent),
    NoExpose(xlib::XNoExposeEvent),
    Property(xlib::XPropertyEvent),
    Reparent(xlib::XReparentEvent),
    ResizeRequest(xlib::XResizeRequestEvent),
    SelectionClear(xlib::XSelectionClearEvent),
    Selection(xlib::XSelectionEvent),
    SelectionRequest(xlib::XSelectionRequestEvent),
    Unmap(xlib::XUnmapEvent),
    Visibility(xlib::XVisibilityEvent),
    None,
}

fn get_kind(event: XEvent) -> EventKind {
    unsafe {
        let kind = event.as_ref().unwrap().get_type();
        match kind {
            xlib::KeymapNotify => EventKind::Keymap(event.as_ref().unwrap().keymap),
            xlib::Expose => EventKind::Expose(event.as_ref().unwrap().expose),
            xlib::GraphicsExpose => {
                EventKind::GraphicsExpose(event.as_ref().unwrap().graphics_expose)
            }
            xlib::NoExpose => EventKind::NoExpose(event.as_ref().unwrap().no_expose),
            xlib::VisibilityNotify => EventKind::Visibility(event.as_ref().unwrap().visibility),
            xlib::CreateNotify => EventKind::CreateWindow(event.as_ref().unwrap().create_window),
            xlib::DestroyNotify => EventKind::DestroyWindow(event.as_ref().unwrap().destroy_window),
            xlib::UnmapNotify => EventKind::Unmap(event.as_ref().unwrap().unmap),
            xlib::MapNotify => EventKind::Map(event.as_ref().unwrap().map),
            20 => EventKind::MapRequest(event.as_ref().unwrap().map_request),
            21 => EventKind::Reparent(event.as_ref().unwrap().reparent),
            22 => EventKind::Configure(event.as_ref().unwrap().configure),
            23 => EventKind::ConfigureRequest(event.as_ref().unwrap().configure_request),
            24 => EventKind::Gravity(event.as_ref().unwrap().gravity),
            25 => EventKind::ResizeRequest(event.as_ref().unwrap().resize_request),
            26 => EventKind::Circulate(event.as_ref().unwrap().circulate),
            27 => EventKind::CirculateRequest(event.as_ref().unwrap().circulate_request),
            28 => EventKind::Property(event.as_ref().unwrap().property),
            29 => EventKind::SelectionClear(event.as_ref().unwrap().selection_clear),
            30 => EventKind::SelectionRequest(event.as_ref().unwrap().selection_request),
            31 => EventKind::Selection(event.as_ref().unwrap().selection),
            32 => EventKind::Colormap(event.as_ref().unwrap().colormap),
            33 => EventKind::ClientMessage(event.as_ref().unwrap().client_message),
            34 => EventKind::Mapping(event.as_ref().unwrap().mapping),
            _ => EventKind::None,
        }
    }
}

pub struct Event {
    inner: XEvent,
    event: EventKind,
}

impl Event {
    pub fn from_raw(event: XEvent) -> Self {
        let event_kind = get_kind(event);

        Self {
            inner: event,
            event: event_kind,
        }
    }

    pub fn as_raw(&self) -> XEvent {
        self.inner
    }

    pub fn get_kind(&self) -> &EventKind {
        &self.event
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { libc::free(self.inner as *mut libc::c_void) };
    }
}
