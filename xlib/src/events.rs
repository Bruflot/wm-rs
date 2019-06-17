use libc;
use std::ops::BitOr;
use x11::xlib;

#[derive(Debug, PartialEq)]
pub enum Events {
    CreateWindowEvent,
    DestroyWindowEvent,
    XButtonEvent,
}

#[derive(Debug, PartialEq)]
pub enum EventMask {
    StructureNotifyMask = 0x0002_0000,
    ResizeRedirectMask = 0x0004_0000,
    SubstructureNotifyMask = 0x0008_0000,
    SubstructureRedirectMask = 0x0010_0000,
}

impl BitOr for EventMask {
    type Output = i64;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as i64 | rhs as i64
    }
}

/// An x11 Event
pub struct Event {
    inner: *mut xlib::XAnyEvent,
    kind: EventKind,
}

/// Type of event
pub enum EventKind {
    CreateWindowEvent,
    DestroyWindowEvent,
    XButtonEvent,
    None,
}

impl Event {
    /// Returns the EventKind of this event
    pub fn kind(&self) -> &EventKind {
        &self.kind
    }

    pub(super) fn from_raw(event: *mut xlib::XAnyEvent) -> Self {
        Event {
            inner: event,
            kind: EventKind::None,
        }
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { libc::free(self.inner as *mut libc::c_void) };
    }
}
