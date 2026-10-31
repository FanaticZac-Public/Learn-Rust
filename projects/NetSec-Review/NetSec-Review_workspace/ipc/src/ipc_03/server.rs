// See readme README_IPC.md for program descriptions

// Based on example from the UnixListener API but with clientId and hashmap
use crate::ipc_03::com_proc::*;
use serde_json;
use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, BufReader, Write},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    thread,
};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

pub fn run_server(socket_path: &str) -> io::Result<()> {
    let path = Path::new(socket_path);

    if path.exists() {
        fs::remove_file(path)?;
    }

    let listener = UnixListener::bind(path)?;
    eprintln!("Listening on {0}", socket_path);

    // OK - We have to shift to mpsc (multi-producer, single-consumer)
    let (connection_tx, connection_rx) = mpsc::channel();

    // We move connection handling to new thread
    thread::spawn(move || {
        for connection in listener.incoming() {
            match connection {
                Ok(stream) => {
                    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

                    println!("Accepted connection from {id}");

                    if connection_tx.send((id, stream)).is_err() {
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("Error accepting connection: {err}");
                    break;
                }
            }
        }
    });

    // The main thread owns the connections and handles all terminal input.
    let mut clients: HashMap<u64, BufReader<UnixStream>> = HashMap::new();
    let stdin = io::stdin();
    let mut input = String::new();

    loop {

        println!("\nActive connections:");
        if clients.is_empty() {
            println!("  (none)");
        } else {
            for id in clients.keys() {
                println!("  {id}");
            }
        }

        println!("Enter a client ID to select it, or q to quit:");
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            break;
        }

        while let Ok((id, stream)) = connection_rx.try_recv() {
            println!("Accepted connection from {id}");
            clients.insert(id, BufReader::new(stream));
        }

        let selection = input.trim();
        if selection == "q" || selection == "quit" {
            break;
        }

        let id = match selection.parse::<u64>() {
            Ok(id) if clients.contains_key(&id) => id,
            _ => {
                println!("That client ID is not in the active list.");
                continue;
            }
        };

        let disconnected = {
            let reader = clients.get_mut(&id).expect("Client does not exist");
            handle_selected_client(reader)?
        };

        if disconnected {
            println!("Client {id} disconnected.");
            clients.remove(&id);
        }
    }

    fs::remove_file(path)?;
    Ok(())
}

fn handle_selected_client(reader: &mut BufReader<UnixStream>) -> io::Result<bool> {
    let stdin = io::stdin();
    let mut input = String::new();
    let mut response = String::new();

    loop {
        println!(
            "\nChoose a command:\n\
             1. Ping\n\
             2. Show notification\n\
             3. Show image\n\
             4. Play sound\n\
             b. Back to client list"
        );
        print!("> ");
        io::stdout().flush()?;

        input.clear();
        if stdin.read_line(&mut input)? == 0 {
            return Ok(false);
        }

        let command = match input.trim() {
            "1" => ServerCommand::Ping,
            "2" => ServerCommand::ShowNotification {
                message: "Hello from the server!".to_owned(),
            },
            "3" => ServerCommand::ShowImage {
                path: "images/example.png".to_owned(),
            },
            "4" => ServerCommand::PlaySound {
                path: "sounds/example.mp3".to_owned(),
            },
            "b" | "back" => return Ok(false),
            _ => {
                println!("Enter 1, 2, 3, 4, or b.");
                continue;
            }
        };

        let mut line = serde_json::to_string(&command)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        line.push('\n');

        reader.get_mut().write_all(line.as_bytes())?;
        reader.get_mut().flush()?;

        response.clear();
        if reader.read_line(&mut response)? == 0 {
            return Ok(true);
        }

        match serde_json::from_str::<CommandResult>(response.trim()) {
            Ok(result) => println!("Client response: {result:?}"),
            Err(e) => eprintln!("Could not parse client response: {e}"),
        }
    }
}
