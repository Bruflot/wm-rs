use x11::xlib;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
// pub fn XOpenDisplay2<A>(display_name: A) where A: Into<CString>{}

pub type Display = *mut xlib::Display;

pub fn XOpenDisplay<A: Into<CString>>(display_name: Option<A>) -> Display{
    let c_str: *const c_char = if display_name.is_some(){
        display_name.unwrap().into().as_ptr()
    } else {
        ptr::null()
    };
    unsafe { xlib::XOpenDisplay(c_str) }
} 

