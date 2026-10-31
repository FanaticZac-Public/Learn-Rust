// See readme README_IPC.md for program descriptions
use crate::ipc_04::com_proc::*;
use serde_json;
use std::{
    io::{self, BufRead, BufReader, Write},
    os::unix::net::UnixStream,
    thread,
    time::Duration,
};

pub fn run_client(socket_path: &str) -> io::Result<()> {
    // let stream = UnixStream::connect(socket_path)?;

    loop {
        let mut stream = connect(socket_path)?;

        match communicate(&mut stream) {
            Ok(()) => eprintln!("Server disconnected; reconnecting"),
            Err(err) => eprintln!("Connection failed: {err}; reconnecting"),
        }

        thread::sleep(Duration::from_secs(1));
    }
}

// We handle connection for retry, the -> communicate
fn connect(path: &str) -> io::Result<UnixStream> {
    loop {
        match UnixStream::connect(path) {
            Ok(stream) => {
                eprintln!("Connection established with {path}");
                return Ok(stream);
            }
            Err(err) => {
                eprintln!("Can't connect to {path}: {err}; retrying...");
                thread::sleep(Duration::from_secs(5));
            }
        }
    }
}

fn communicate(stream: &mut UnixStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    loop {
        line.clear();

        // Read one command, ending at the newline written by the server.
        if reader.read_line(&mut line)? == 0 {
            // Server closed the connection.
            return Ok(());
        }

        let command: ServerCommand = match serde_json::from_str(line.trim()) {
            Ok(command) => command,
            Err(e) => {
                eprintln!("Could not parse server command: {e}");
                continue;
            }
        };

        let result = handle_command(command);

        let mut response = serde_json::to_string(&result)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        response.push('\n');

        reader.get_mut().write_all(response.as_bytes())?;
        reader.get_mut().flush()?;
    }
}

fn handle_command(command: ServerCommand) -> CommandResult {
    match command {
        ServerCommand::Ping => {
            println!("Received Ping");
            CommandResult::Pong
        }
        ServerCommand::ShowNotification { message } => {
            println!("Notification: {message}");
            CommandResult::NotificationShown
        }
        ServerCommand::ShowImage { filename, data } => {
            println!("Show image: {filename}, size: {} bytes", data.len());
            CommandResult::ImageShown
        }
        ServerCommand::PlaySound { filename, data } => {
            println!("Play sound: {filename}, size: {} bytes", data.len());
            CommandResult::SoundStarted
        }
    }
}
