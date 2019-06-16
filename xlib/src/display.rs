use crate::errors::XError;
use std::ffi::CString;
use std::ptr;
use x11::xlib;

pub const SUBSTRUCTURE_REDIRECT_MASK: i64 = 0x0008_0000;
pub const SUBSTRUCTURE_NOTIFY_MASK: i64 = 0x0010_0000;
pub type Window = *mut xlib::Window;
pub type Display = *mut xlib::Display;

#[derive(Debug)]
pub enum Error {
    BadAccess,
    BadWindow,
}

#[derive(Debug)]
pub enum Event {
    CreateNotify,
    DestroyNotify,
    ConfigureRequest,
    MapRequest,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ErrorEvent {
    event_type: i32,
    resource_id: u64,
    serial: u64,
    error_code: u8,
    request_code: u8,
    minor_code: u8,
}

#[derive(Debug)]
pub struct Connection {
    display: Display,
    // root: Window,
    // error_handler: Option<fn(ErrorEvent)>,
}

impl Connection {
    pub fn connect<T: AsRef<str>>(display_name: T) -> Result<Self, XError> {
        let display_name = CString::new(display_name.as_ref()).unwrap();
        let display = unsafe { xlib::XOpenDisplay(display_name.as_ptr()) };

        if display == ptr::null_mut() {
            return Err(XError::OpenDisplayError);
        }

        Ok(Self { display })
    }

    // XRootWindowOfScreen

    // XDefaultScreenOfDisplay
    pub fn default_screen(&self){

    }

    // XDefaultRootWindow
    pub fn default_window(&self) -> u64{
        unsafe { xlib::XDefaultRootWindow(self.display) }
    }

    

    // pub fn select_input(&self, event_mask: i64) -> Result<(), Error>{
    //     unsafe{
    //         let r = XSelectInput(self.display, self.root, event_mask);
    //         if r == BadWindow as i32{
    //             return Err(Error::BadWindow);
    //         }
    //         Ok(())
    //     }
    // }

    // pub fn sync(&self, discard: bool){
    //     unsafe{ XSync(self.display, discard as i32); }
    // }

    // pub fn set_error_handler(&self, error_handler: &Fn(ErrorEvent)){
    //     // todo: error?
    //     unsafe { XSetErrorHandler(Some(Connection::wm_error)); }
    // }

    // pub fn create_window(&self, x: i32, y: i32, width: u32, height: u32, border: u32,
    //     border_color: u64, bg_color: u64) -> u64{
    //     unsafe{ XCreateSimpleWindow(self.display, self.root, x, y, width, height, border, border_color, bg_color) }
    // }

    // pub fn map_request(&self, window: Window){
    //     unsafe{ XMapWindow(self.display, window); }
    // }

    // unsafe extern "C" fn wm_error(display: Display, event: *mut XErrorEvent) -> i32{
    //     if (*event).error_code == BadAccess{
    //         panic!("BadAccess - A WM is already running! Exiting...");
    //     }
    //     return 0;
    // }
}
