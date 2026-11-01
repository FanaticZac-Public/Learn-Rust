// See readme README_IPC.md for program descriptions
use std::{fs::File, io, os::fd::AsRawFd};

use uds::{UnixSeqpacketConn, UnixSocketAddr};

pub fn run_client() -> io::Result<()> {
    let addr = UnixSocketAddr::from_abstract(b"rust-fd-demo")?;
    let conn = UnixSeqpacketConn::connect_unix_addr(&addr)?;

    let file = File::open("/etc/hostname")?;
    conn.send_fds(b"Here is a file", &[file.as_raw_fd()])?;

    let mut reply = [0u8; 128];
    let len = conn.recv(&mut reply)?;

    println!("Server replied: {}", String::from_utf8_lossy(&reply[..len]));
    Ok(())
}
