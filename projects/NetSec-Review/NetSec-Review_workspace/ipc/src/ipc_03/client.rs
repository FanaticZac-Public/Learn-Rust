// See readme README_IPC.md for program descriptions
use crate::ipc_03::com_proc::*;
use serde_json;
use std::{
    os::unix::net::UnixStream,
    io::{self, BufRead, BufReader, Write},
};


pub fn run_client(socket_path: &str) -> io::Result<()> {
    let stream = UnixStream::connect(socket_path)?;
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
        },
        ServerCommand::ShowNotification { message } => {
            println!("Notification: {message}");
            CommandResult::NotificationShown
        }
        ServerCommand::ShowImage { path } => {
            println!("Show image: {path}");
            CommandResult::ImageShown
        }
        ServerCommand::PlaySound { path } => {
            println!("Play sound: {path}");
            CommandResult::SoundStarted
        }
    }
}
