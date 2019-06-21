use crate::wm::{EventNotify, EventTx, Rect, Window};

pub struct EventHandler;

impl EventHandler {
    fn map(&self, window: &mut Window) {
        window.move_resize(Rect {
            x: 0,
            y: 0,
            width: 512,
            height: 512,
        });
    }
}

impl EventTx for EventHandler {
    fn notify(&self, event: EventNotify, window: &mut Window) {
        match event {
            EventNotify::Map => self.map(window),
            _ => (),
        }
    }
}
