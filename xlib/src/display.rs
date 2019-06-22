extern crate libc;
use crate::{Event, Window, XDisplay, XError, XEvent, XResult};
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

    pub fn as_raw(&self) -> XDisplay {
        self.inner
    }

    // XSync
    pub fn sync<T: Into<i32>>(&self, discard: T) {
        unsafe {
            xlib::XSync(self.inner, discard.into());
        }
    }

    // XReparentWindow
    pub fn reparent_window(&self, window: &Window, parent: &Window) {
        unsafe {
            xlib::XReparentWindow(self.inner, window.as_raw(), parent.as_raw(), 0, 0);
        }
    }

    // XMapWindow
    pub fn map_window(&self, window: &Window) {
        unsafe {
            xlib::XMapWindow(self.inner, window.as_raw());
        }
    }

    pub fn unmap_window(&self, window: &Window) {
        unsafe {
            xlib::XUnmapWindow(self.inner, window.as_raw());
        }
    }

    // XSelectInput
    pub fn select_input(&self, window: &Window, event_mask: i64) {
        unsafe {
            xlib::XSelectInput(self.inner, window.as_raw(), event_mask);
        }
    }

    // XGrabButton
    pub fn grab_button(&self, window: &Window, button: u32, modifier: Option<u32>) {
        let modifier = modifier.unwrap_or(xlib::AnyModifier);

        unsafe {
            xlib::XGrabButton(
                self.inner,
                button,
                modifier,
                window.as_raw(),
                0,
                (xlib::ButtonPressMask | xlib::ButtonReleaseMask | xlib::ButtonMotionMask) as u32,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
                0,
                0,
            );
        }
    }

    // XGrabKey
    pub fn grab_key(&self, window: &Window, key: char, modifier: Option<u32>) {
        let modifier = modifier.unwrap_or(xlib::AnyModifier);

        unsafe {
            let code = xlib::XKeysymToKeycode(self.inner, key as u64) as i32;
            xlib::XGrabKey(
                self.inner,
                code,
                modifier,
                window.as_raw(),
                0,
                xlib::GrabModeAsync,
                xlib::GrabModeAsync,
            );
        }
    }

    // XNextEvent
    pub fn next_event(&self) -> Event {
        unsafe {
            let event = libc::malloc(mem::size_of::<xlib::XEvent>()) as XEvent;
            xlib::XNextEvent(self.inner, event);
            Event::from_raw(event)
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.inner);
        }
    }
}
