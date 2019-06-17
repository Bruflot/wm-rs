use crate::errors::XError;
use crate::events::{Event, EventMask};
use crate::window::Window;
use crate::XResult;
use std::ffi::CString;
use std::ptr;
use x11::xlib;

type XDisplay = *mut xlib::Display;

#[derive(Debug)]
pub struct Display {
    pub(crate) inner: XDisplay,
}

impl Display {
    // XOpenDisplay -- should be nullable.
    pub fn connect<T: AsRef<str>>(display_name: Option<T>) -> XResult<Display> {
        // pub fn connect(display_name: Option<&str>) -> Result<Display, XError> {
        let display_name = match display_name {
            Some(name) => CString::new(name.as_ref()).unwrap().as_ptr(),
            None => ptr::null(),
        };
        let display = unsafe { xlib::XOpenDisplay(display_name) };

        if display == ptr::null_mut() {
            return Err(XError::OpenDisplayError);
        }

        Ok(Self { inner: display })
    }

    // XDefaultRootWindow
    pub fn default_window(&self) -> Window {
        let window = unsafe { xlib::XDefaultRootWindow(self.inner) };
        Window {
            display: self.inner,
            inner: window,
        }
    }

    // XSync
    pub fn sync<T: Into<i32>>(&self, discard: T) {
        unsafe {
            xlib::XSync(self.inner, discard.into());
        }
    }

    // XMapWindow
    pub fn map_window(&self, window: &Window) -> XResult<()> {
        let ret = unsafe { xlib::XMapWindow(self.inner, window.inner) };
        if ret == xlib::BadWindow as i32 {
            return Err(XError::BadWindow);
        }
        Ok(())
    }

    // XSelectInput
    pub fn select_input(&self, window: &Window, event_mask: EventMask) -> XResult<()> {
        let ret = unsafe { xlib::XSelectInput(self.inner, window.inner, event_mask as i64) };
        if ret == xlib::BadWindow as i32 {
            return Err(XError::BadWindow);
        }
        Ok(())
    }

    pub fn next_event(&self) -> Event {
        let event =
            unsafe { libc::malloc(std::mem::size_of::<xlib::XAnyEvent>()) as *mut xlib::XAnyEvent };
        Event::from_raw(event)
    }

    // XDisplayWidth
    pub fn get_width(&self) -> u32 {
        unsafe { xlib::XDisplayWidth(self.inner, 0) as u32 }
    }

    // XDisplayHeight
    pub fn get_height(&self) -> u32 {
        unsafe { xlib::XDisplayHeight(self.inner, 0) as u32 }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.inner);
        }
    }
}
