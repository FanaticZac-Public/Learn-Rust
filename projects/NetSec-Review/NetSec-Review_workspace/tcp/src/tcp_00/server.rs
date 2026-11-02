// See readme README_IPC.md for program descriptions

use std::{
    fs,
    io::{self, Read, Write},
    os::unix::net::{UnixListener},
    path::Path,
};

pub fn run_server(socket_path: &str) -> io::Result<()> {
    let path = Path::new(socket_path);

    if path.exists() {
        fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;
    let (mut stream, _) = listener.accept()?;

    let mut request = String::new();
    stream.read_to_string(&mut request)?;
    println!("Received: {}", request.trim());

    stream.write_all(b"hello from server\n")?;
    fs::remove_file(path)?;

    Ok(())
}

