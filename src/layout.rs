use crate::wm::{EventNotify, EventRx, Rect, Window, Display};

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

impl EventRx for EventHandler {
    // Register any key/button events you may want to receive
    fn setup(&self, display: &Display, root: &Window){
        display.grab_button(root, 1, None);
        display.grab_key(root, 'a', None);
    }

    fn notify(&self, event: EventNotify, window: &mut Window) {
        match event {
            EventNotify::Map => self.map(window),
            _ => (),
        }
    }
}
