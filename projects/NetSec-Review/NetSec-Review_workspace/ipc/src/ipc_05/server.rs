// See readme README_IPC.md for program descriptions
use std::{
    io,
    os::unix::net::UnixDatagram,
};

pub fn run_server(socket_path: &str) -> io::Result<()> {
    let _ = std::fs::remove_file(socket_path);

    let socket = UnixDatagram::bind(socket_path)?;
    println!("Listening on {socket_path}");

    loop {
        let mut buffer = [0u8; 1024];

        // recv_from 
        let (len, address) = socket.recv_from(&mut buffer)?;

        println!("Received: {}", String::from_utf8_lossy(&buffer[..len]));

        // Just checking some data, error output

        // local_addr()
        println!("Sending address: {:?}", socket.local_addr());

        // peer_addr()
        println!("Peer address: {:?}", socket.peer_addr());

        // for match - but just checking
        println!("Connect address: {:?}", socket.connect_addr(&address));

        if let Some(client_path) = address.as_pathname() {
            socket.send_to(b"Hello from the server", client_path)?;
        }
    }
}
