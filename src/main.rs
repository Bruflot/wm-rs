#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod wm;
use wm::{Events, Rect, WM};

fn main() {
    pretty_env_logger::init();

    let mut wm = WM::new().expect("Failed to connect to X!");
    info!("Connected to X");

    wm.run();

    // loop {
    //     let event = wm.next_event();
    //     let kind = event.kind();
    //     info!("Event received: {:?}", kind);

    //     match kind {
    //         // A new window was created
    //         // We rarely need to handle this as the application creates the
    //         // window itself.
    //         Events::CreateNotify => (),
    //         Events::DestroyNotify => (),
    //         Events::ConfigureRequest => {
    //             info!("Configuring request");
    //         }
    //         _ => warn!("Ignored event"),
    //     }
    // }
}
