#![allow(dead_code)]
use std::fs;
use std::io;
use std::os::unix::net::UnixListener;
use std::path::Path;

pub struct Server {
    listener: UnixListener,
}

impl Server {
    pub fn new<T: AsRef<str>>(path: T) -> io::Result<Self> {
        let socket = path.as_ref();
        if Path::new(socket).exists() {
            info!("Unlinking old socket file");
            fs::remove_file(path.as_ref())?;
        }

        let listener = UnixListener::bind(socket)?;
        info!("Listening to socket {}", socket);

        Ok(Self { listener })
    }

    fn incoming() {
        // todo: protocol
    }
}
