// See readme README_IPC.md for program descriptions
use std::{
    io,
    os::unix::net::UnixDatagram,
};

pub fn run_client(socket_path: &str, server_socket_path: &str) -> io::Result<()> {
    let _ = std::fs::remove_file(socket_path);

    let socket = UnixDatagram::bind(socket_path)?;
    socket.send_to(b"Hello from the client", server_socket_path)?;

    let mut buffer = [0u8; 1024];
    let len = socket.recv(&mut buffer)?;

    println!("Reply: {}", String::from_utf8_lossy(&buffer[..len]));

    drop(socket);

    let _ = std::fs::remove_file(socket_path);

    Ok(())
}
