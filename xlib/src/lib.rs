extern crate x11;

use std::ptr;
use x11::xlib::*;

pub const SubstructureNotifyMask: i64 = 0x0008_0000;
pub const SubstructureRedirectMask: i64 = 0x0010_0000;
pub type Window = x11::xlib::Window;
pub type Display = *mut x11::xlib::Display;

#[derive(Debug)]
pub enum Error{
    BadAccess,
    BadWindow,
}

#[derive(Debug)]
pub enum Event{
    CreateNotify,
    DestroyNotify,
    ConfigureRequest,
    MapRequest,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErrorEvent{
    event_type: i32,
    resource_id: u64,
    serial: u64,
    error_code: u8,
    request_code: u8,
    minor_code: u8,
}

#[derive(Debug)]
pub struct Connection{
    display: Display,
    root: Window,
    error_handler: Option<fn(ErrorEvent)>,
}

impl Connection{
    pub fn connect() -> Result<Connection, Error>{
        let display: Display;
        let root: Window;

        unsafe{
            display = XOpenDisplay(ptr::null());

            if display == ptr::null_mut(){
                return Err(Error::BadAccess);
            }

            root = XDefaultRootWindow(display);
            XSetErrorHandler(Some(Connection::wm_error));
        }
        
        return Ok(Connection{
            display: display,
            root: root,
            error_handler: None,
        });
    }

    pub fn select_input(&self, event_mask: i64) -> Result<(), Error>{
        unsafe{ 
            let r = XSelectInput(self.display, self.root, event_mask);
            if r == BadWindow as i32{
                return Err(Error::BadWindow);
            }
            Ok(())
        }
    }

    pub fn sync(&self, discard: bool){
        unsafe{ XSync(self.display, discard as i32); }
    }

    pub fn set_error_handler(&self, error_handler: &Fn(ErrorEvent)){
        // todo: error? 
        unsafe { XSetErrorHandler(Some(Connection::wm_error)); }
    }

    pub fn create_window(&self, x: i32, y: i32, width: u32, height: u32, border: u32, 
        border_color: u64, bg_color: u64) -> u64{
        unsafe{ XCreateSimpleWindow(self.display, self.root, x, y, width, height, border, border_color, bg_color) }
    }

    pub fn map_request(&self, window: Window){
        unsafe{ XMapWindow(self.display, window); }
    }

    unsafe extern "C" fn wm_error(display: Display, event: *mut XErrorEvent) -> i32{
        if (*event).error_code == BadAccess{
            panic!("BadAccess - A WM is already running! Exiting...");
        }
        return 0;
    }
 
}
