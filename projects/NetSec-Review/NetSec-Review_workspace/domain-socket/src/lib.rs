// For Unix/Linux domain sockets the 2 libraries relevant are:
// https://doc.rust-lang.org/std/os/unix/net/struct.UnixListener.html
// https://doc.rust-lang.org/std/os/unix/net/struct.UnixStream.html

use std::{
    fs,
    io::{self, Read, Write},
    os::unix::net::{UnixListener, UnixStream},
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

pub fn run_client(socket_path: &str) -> io::Result<String> {
    let mut stream = UnixStream::connect(socket_path)?;

    stream.write_all(b"hello from client\n")?;
    stream.shutdown(std::net::Shutdown::Write)?;

    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}


// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
