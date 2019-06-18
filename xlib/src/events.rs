use crate::XEvent;
use std::ops::BitOr;

#[derive(Debug, PartialEq)]
pub enum Events {
    CreateNotify,
    DestroyNotify,
    MapNotify,
    UnmapNotify,
    Other,
}

pub struct Event {
    inner: XEvent,
}

impl Event {
    pub fn from_raw(event: XEvent) -> Self {
        Self { inner: event }
    }

    pub fn kind(&self) -> Events {
        let kind = unsafe { self.inner.as_ref().unwrap().get_type() };
        match kind {
            16 => Events::CreateNotify,
            17 => Events::DestroyNotify,
            18 => Events::UnmapNotify,
            19 => Events::MapNotify,
            _ => Events::Other,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum EventMask {
    KeyPressMask = 0x0000_0001,
    KeyPressRelease = 0x0000_0002,
    ButtonPressMask = 0x0000_0004,
    ButtonReleaseMask = 0x0000_0008,
    SubstructureNotifyMask = 0x0008_0000,
}

impl BitOr for EventMask {
    type Output = i64;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as i64 | rhs as i64
    }
}
