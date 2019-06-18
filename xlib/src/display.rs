extern crate libc;
use crate::events::{Event, EventMask};
use crate::{Window, XDisplay, XError, XResult};
use std::ffi::CString;
use std::mem;
use std::ptr;
use x11::xlib;

#[derive(Debug)]
pub struct Display {
    inner: XDisplay,
}

impl Display {
    // XOpenDisplay
    pub fn connect(display_name: Option<&str>) -> XResult<Display> {
        let display_name = match display_name {
            Some(name) => CString::new(name).unwrap().as_ptr(),
            None => ptr::null(),
        };
        let display = unsafe { xlib::XOpenDisplay(display_name) };

        if display.is_null() {
            return Err(XError::ConnectionError);
        }
        Ok(Self { inner: display })
    }

    // XDefaultRootWindow
    pub fn default_window(&self) -> Window {
        let window = unsafe { xlib::XDefaultRootWindow(self.inner) };
        Window::from_raw(self, window)
    }

    // XSync
    pub fn sync<T: Into<i32>>(&self, discard: T) {
        unsafe {
            xlib::XSync(self.inner, discard.into());
        }
    }

    // XMapWindow
    pub fn map_window(&self, window: &Window) {
        unsafe {
            xlib::XMapWindow(self.inner, window.as_raw());
        }
    }

    // XSelectInput
    pub fn select_input(&self, window: &Window, event_mask: EventMask) {
        unsafe {
            xlib::XSelectInput(self.inner, window.as_raw(), event_mask as i64);
        }
    }

    // XNextEvent
    pub fn next_event(&self) -> Event {
        let event = unsafe {
            let e = libc::malloc(mem::size_of::<xlib::XEvent>()) as *mut xlib::XEvent;
            xlib::XNextEvent(self.inner, e);
            e
        };
        Event::from_raw(event)
    }

    // XDisplayWidth
    pub fn get_width(&self) -> i32 {
        unsafe { xlib::XDisplayWidth(self.inner, 0) }
    }

    // XDisplayHeight
    pub fn get_height(&self) -> i32 {
        unsafe { xlib::XDisplayHeight(self.inner, 0) }
    }

    pub fn as_raw(&self) -> XDisplay {
        self.inner
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.inner);
        }
    }
}
