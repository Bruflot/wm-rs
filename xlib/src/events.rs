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
    Other,
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
            xlib::MapRequest => EventKind::MapRequest(event.as_ref().unwrap().map_request),
            xlib::ReparentNotify => EventKind::Reparent(event.as_ref().unwrap().reparent),
            xlib::ConfigureNotify => EventKind::Configure(event.as_ref().unwrap().configure),
            xlib::ConfigureRequest => {
                EventKind::ConfigureRequest(event.as_ref().unwrap().configure_request)
            }
            xlib::GravityNotify => EventKind::Gravity(event.as_ref().unwrap().gravity),
            xlib::ResizeRequest => EventKind::ResizeRequest(event.as_ref().unwrap().resize_request),
            xlib::CirculateNotify => EventKind::Circulate(event.as_ref().unwrap().circulate),
            xlib::CirculateRequest => {
                EventKind::CirculateRequest(event.as_ref().unwrap().circulate_request)
            }
            xlib::PropertyNotify => EventKind::Property(event.as_ref().unwrap().property),
            xlib::SelectionClear => {
                EventKind::SelectionClear(event.as_ref().unwrap().selection_clear)
            }
            xlib::SelectionRequest => {
                EventKind::SelectionRequest(event.as_ref().unwrap().selection_request)
            }
            xlib::SelectionNotify => EventKind::Selection(event.as_ref().unwrap().selection),
            xlib::ColormapNotify => EventKind::Colormap(event.as_ref().unwrap().colormap),
            xlib::ClientMessage => EventKind::ClientMessage(event.as_ref().unwrap().client_message),
            xlib::MappingNotify => EventKind::Mapping(event.as_ref().unwrap().mapping),
            _ => EventKind::Other,
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
