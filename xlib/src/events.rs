extern crate libc;
extern crate x11;

use crate::{XDisplay, XEvent};
use std::ops::BitOr;
use x11::xlib;

#[derive(Debug, PartialEq)]
pub enum Events {
    CreateNotify,
    DestroyNotify,
    MapNotify,
    UnmapNotify,
    MapRequest,
    ReparentNotify,
    ConfigureNotify,
    ConfigureRequest,
    Other,
}

pub struct Event {
    inner: XEvent,
    kind: Events,
    display: XDisplay,
}

fn get_kind(event: i32) -> Events{
    match event{
        16 => Events::CreateNotify,
        17 => Events::DestroyNotify,
        18 => Events::UnmapNotify,
        19 => Events::MapNotify,
        20 => Events::MapRequest,
        21 => Events::ReparentNotify,
        22 => Events::ConfigureNotify,
        23 => Events::ConfigureRequest,
        _ => Events::Other,
    }
}

impl Event {
    pub fn from_raw(event: XEvent) -> Self{
        let fields = unsafe { event.as_ref().unwrap().any };
        let kind = unsafe { event.as_ref().unwrap().get_type() };

        Self{
            inner: event,
            kind: get_kind(kind),
            display: fields.display,
        }
    }

    pub fn as_raw(&self) -> XEvent {
        self.inner
    }

    pub fn get_kind(&self) -> &Events {
        &self.kind
    }

    pub fn get_display(&self) -> XDisplay{
        self.display
    }

    pub fn get_map_event(&self) -> xlib::XMapRequestEvent{
        unsafe { self.inner.as_ref().unwrap().map_request }   
    }
}

impl Drop for Event {
    fn drop(&mut self) {
        unsafe { libc::free(self.inner as *mut libc::c_void) };
    }
}

#[derive(Debug, PartialEq)]
pub enum EventMask {
    KeyPressMask = 0x0000_0001,
    KeyPressRelease = 0x0000_0002,
    ButtonPressMask = 0x0000_0004,
    ButtonReleaseMask = 0x0000_0008,
    SubstructureNotifyMask = 0x0008_0000,
    SubstructureRedirectMask = 0x0010_0000,
}

impl Into<i64> for EventMask {
    fn into(self) -> i64 {
        self as i64
    }
}

impl BitOr for EventMask {
    type Output = i64;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as i64 | rhs as i64
    }
}
