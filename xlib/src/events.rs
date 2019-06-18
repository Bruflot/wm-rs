use std::ops::BitOr;
use x11::xlib;

#[derive(Debug, PartialEq)]
pub enum Events {
    CreateWindowEvent,
    DestroyWindowEvent,
    XButtonEvent,
}

#[derive(Debug, PartialEq)]
pub enum EventMask {
    StructureNotifyMask = 0x0002_0000,
    ResizeRedirectMask = 0x0004_0000,
    SubstructureNotifyMask = 0x0008_0000,
    SubstructureRedirectMask = 0x0010_0000,
}

impl BitOr for EventMask {
    type Output = i64;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as i64 | rhs as i64
    }
}
