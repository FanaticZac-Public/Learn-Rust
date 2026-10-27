// See readme README_IPC.md for program descriptions
// Based on example from the UnixListener API but with clientId and hashmap
use std::{
    fs,
    io::{self, BufRead, BufReader},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    thread,
    sync::atomic::{AtomicU64, Ordering},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

// Added client info
struct Client {
    id: u64,
    stream: UnixStream,
}

pub fn run_server(socket_path: &str) -> io::Result<()> {
    let path = Path::new(socket_path);

    if path.exists() {
        fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;
    eprintln!("Listening on {0}", socket_path);

    // We will now user listener.incoming to parse clients into new threads
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // shows fd but not reliable id indicator (arbitrary for client process table
                // println!("{0:?}", stream);
                // UnixStream { fd: FileDesc(OwnedFd { fd: 5 }), local: "/tmp/domain-socket.sock" (pathname), peer: (unnamed) }

                // static id handling
                let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
                let client = Client { id, stream };

                println!("Accepted connection from {id}");

                thread::spawn(move || {
                    let _ = handle_client(client);
                });
            }
            Err(err) => {
                println!("Err: {err}");
                break;
            }
        }
    }

    // stream.write_all(b"hello from server\n")?;
    fs::remove_file(path)?;

    Ok(())
}

// We will handle client here now
fn handle_client(client: Client) -> io::Result<()> {
    // MOVED HERE
    // BufReader use '\n' as the message termination indicator.
    let mut reader = BufReader::new(client.stream);
    let mut message = String::new();

    loop {
        message.clear();

        if reader.read_line(&mut message)? == 0 {
            break;
        }

        print!("{0} says: {message}", client.id);
    }
    Ok(())
}
