extern crate xlib;
extern crate x11;

use xlib::*;

struct WindowManager{
    connection: Connection,
}

impl WindowManager{
    pub fn new() -> Self{
        let con = Connection::connect().expect("Failed to connect to an X server!");
        con.select_input(SubstructureNotifyMask | SubstructureRedirectMask);
        con.sync(false);

        WindowManager{
            connection: con,
        }
    }

    pub fn frame(&self, window: Window) -> Window{
        let frame = self.connection.create_window(0, 0, 300, 300, 4, 0xffffff, 0x000000);
        frame
    }

    pub fn map(&self, window: Window){
        self.connection.map_request(window);
    }

    pub fn run(&self){
        loop{
            
        }
    }
}

fn main(){
    let wm = WindowManager::new().run();
}
