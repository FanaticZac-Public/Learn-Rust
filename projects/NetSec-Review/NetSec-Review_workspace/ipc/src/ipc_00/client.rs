// See readme README_IPC.md for program descriptions

use std::{
    fs,
    io::{self, Read, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
};

pub fn run_client(socket_path: &str) -> io::Result<String> {
    let mut stream = UnixStream::connect(socket_path)?;

    stream.write_all(b"hello from client\n")?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
} 