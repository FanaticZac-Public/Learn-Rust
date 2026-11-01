// See readme README_IPC.md for program descriptions
use std::{fs::File, io, io::Read, os::fd::FromRawFd};
use uds::{UnixSeqpacketListener, UnixSocketAddr};

pub fn run_server() -> io::Result<()> {
    // Abstract address
    let addr = UnixSocketAddr::from_abstract(b"rust-fd-demo")?;
    // SOCK_SEQPACKET socket
    let listener = UnixSeqpacketListener::bind_unix_addr(&addr)?;

    // Connection setup - just awaiting one client example
    println!("Waiting for client...");
    let (conn, _) = listener.accept_unix_addr()?;

    // Ok - so the goal here is:
    // Passing fd - aka passing open resource -
    let mut message = [0u8; 128];
    let mut received_fds = [-1; 1];

    let (len, truncated, fd_count) = conn.recv_fds(&mut message, &mut received_fds)?;

    if truncated {
        return Err(std::io::Error::other("message was truncated"));
    }
    if fd_count != 1 {
        return Err(std::io::Error::other("expected exactly one FD"));
    }

    println!("Message: {}", String::from_utf8_lossy(&message[..len]));

    // recv_fds returns a raw FD that the receiver must close.

    // File takes ownership and closes it automatically when dropped.
    let mut file = unsafe { File::from_raw_fd(received_fds[0]) };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("File contents: {contents}");

    conn.send(b"Got it")?;
    Ok(())
}
