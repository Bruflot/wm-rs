#![allow(dead_code, non_snake_case)]
use std::ffi::CString;
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_ulong};
use std::ptr;
use x11::xlib;

pub type XDisplay = *mut xlib::Display;
pub type XWindow = c_ulong;
pub type XEvent = *mut xlib::XEvent;

const BAD_ALLOC: c_ulong = xlib::BadAccess as c_ulong;
const BAD_MATCH: c_ulong = xlib::BadMatch as c_ulong;
const BAD_VALUE: c_ulong = xlib::BadValue as c_ulong;
const BAD_WINDOW: c_ulong = xlib::BadWindow as c_ulong;
pub const STRUCTURE_NOTIFY_MASK: c_long = xlib::SubstructureNotifyMask;
pub const STRUCTURE_REDIRECT_MASK: c_long = xlib::SubstructureRedirectMask;

// Into *const c_char instead?
pub fn XOpenDisplay<A: Into<CString>>(display_name: Option<A>) -> Option<XDisplay> {
    let c_str: *const c_char = if display_name.is_some() {
        display_name.unwrap().into().as_ptr()
    } else {
        ptr::null()
    };

    let display = unsafe { xlib::XOpenDisplay(c_str) };
    if display == ptr::null_mut() {
        Some(display)
    } else {
        None
    }
}

pub fn XDefaultRootWindow(display: XDisplay) -> XWindow {
    unsafe { xlib::XDefaultRootWindow(display) }
}

pub fn XCreateSimpleWindow(
    display: XDisplay,
    parent: XWindow,
    x: c_int,
    y: c_int,
    width: c_uint,
    height: c_uint,
    border: c_ulong,
    background: c_ulong,
) -> Option<c_ulong> {
    let result = unsafe {
        xlib::XCreateSimpleWindow(display, parent, x, y, width, height, 0, border, background)
    };

    match result {
        BAD_ALLOC | BAD_MATCH | BAD_VALUE | BAD_WINDOW => None,
        x @ _ => Some(x),
    }
}

pub fn XSelectInput(display: XDisplay, window: XWindow, event_mask: c_long) -> Option<()> {
    let result = unsafe { xlib::XSelectInput(display, window, event_mask) };
    match result as c_ulong {
        BAD_WINDOW => None,
        _ => Some(()),
    }
}

pub fn XMapWindow(display: XDisplay, window: XWindow) -> Option<()> {
    let result = unsafe { xlib::XMapWindow(display, window) };
    match result as c_ulong {
        BAD_WINDOW => None,
        _ => Some(()),
    }
}

pub fn XNextEvent(display: XDisplay) -> XEvent {
    let event;
    unsafe {
        event = mem::uninitialized();
        xlib::XNextEvent(display, event);
    }
    event
}

pub fn XFlush(display: XDisplay) {
    unsafe {
        xlib::XFlush(display);
    }
}

pub fn XSync(display: XDisplay, discard: bool) {
    unsafe {
        xlib::XSync(display, discard as c_int);
    }
}
