// See readme README_IPC.md for program descriptions

use std::{
    fs,
    io::{self, BufRead, BufReader},
    os::unix::net::{UnixListener},
    path::Path,
};

pub fn run_server(socket_path: &str) -> io::Result<()> {
    let path = Path::new(socket_path);

    if path.exists() {
        fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;
    let (stream, _) = listener.accept()?;
    let mut reader = BufReader::new(stream);
    let mut message = String::new();

    // This time we use BufReader to use '\n' as the message termination indicator.

    loop {
        message.clear();

        if reader.read_line(&mut message)? == 0 {
            break;
        }

        print!("Received: {message}");
    }

    // stream.write_all(b"hello from server\n")?;
    fs::remove_file(path)?;

    Ok(())
}
