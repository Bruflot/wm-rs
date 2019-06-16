extern crate x11;

mod display;
mod errors;
mod window;

pub use display::Display;
pub use errors::XError;
pub type Result<T> = std::result::Result<T, XError>;