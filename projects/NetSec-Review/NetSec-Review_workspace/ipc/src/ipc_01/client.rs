// See readme README_IPC.md for program descriptions

use std::{
    fs,
    io::{self, Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
};

pub fn run_client(socket_path: &str) -> io::Result<String> {
    let mut stream = UnixStream::connect(socket_path)?;

    let mut message = String::new();
    loop {
        message.clear();

        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");

        stream.write_all(message.as_bytes())?;
    }

    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}
