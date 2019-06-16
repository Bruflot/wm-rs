use crate::errors::XError;
use crate::window::Window;
use std::ffi::CString;
use std::ptr;
use x11::xlib;

pub type XDisplay = *mut xlib::Display;

#[derive(Copy, Clone, Debug)]
pub struct Display {
    pub(crate) display: XDisplay,
}

impl Display {
    // XOpenDisplay -- should be nullable.
    pub fn connect<T: AsRef<str>>(display_name: Option<T>) -> Result<Display, XError> {
        let display_name = match display_name {
            Some(name) => CString::new(name.as_ref()).unwrap().as_ptr(),
            None => ptr::null(),
        };
        let display = unsafe { xlib::XOpenDisplay(display_name) };

        if display == ptr::null_mut() {
            return Err(XError::OpenDisplayError);
        }

        Ok(Self { display })
    }

    // XDefaultScreenOfDisplay
    pub fn default_screen(&self) {}

    // XDefaultRootWindow
    pub fn default_window(&self) -> Window {
        let window = unsafe { xlib::XDefaultRootWindow(self.display) };
        Window {
            display: self.display,
            inner: window,
        }
    }

    // XSync
    pub fn sync<T: Into<i32>>(&self, discard: T) {
        unsafe {
            xlib::XSync(self.display, discard.into());
        }
    }

    // TODO: errors
    pub fn map_window(&self, window: &Window) {
        unsafe {
            xlib::XMapWindow(self.display, window.inner);
        }
    }

    // XDisplayWidth
    pub fn get_width(&self) -> u32 {
        unsafe { xlib::XDisplayWidth(self.display, 0) as u32 }
    }

    // XRootWindowOfScreen
}
