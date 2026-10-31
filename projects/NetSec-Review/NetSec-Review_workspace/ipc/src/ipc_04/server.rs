// See readme README_IPC.md for program descriptions

use crate::ipc_04::com_proc::*;
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

    // mpsc (multi-producer, single-consumer) - Thread communication
    // - doubled up - one for inputs - one for connections
    // My main look can now update both independently
    // - and respond to change for smooth user cmd interface
    let (connection_tx, connection_rx) = mpsc::channel::<(u64, UnixStream)>();
    let (input_tx, input_rx) = mpsc::channel::<String>();
    let mut clients: HashMap<u64, BufReader<UnixStream>> = HashMap::new();

    // Thread for Connections
    thread::spawn(move || {
        for connection in listener.incoming() {
            match connection {
                Ok(stream) => {
                    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);

                    // println!("Accepted connection from {id}");

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

    // Thread for Inputs
    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(line) => {
                    if input_tx.send(line).is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    });

    loop {
        println!("Active connections:");
        for client_id in clients.keys() {
            print!("  {client_id}");
        }
        print!("\nSelect ID, 'a' for all, or 'q' to quit:");
        print!("\n> ");

        io::stdout().flush()?;

        // Wait for user input, but check for new connections every 200 ms.
        let selection = 'main: loop {
            // < first use of teh loop labelling feature of rust
            match connection_rx.recv_timeout(std::time::Duration::from_millis(200)) {
                Ok((id, stream)) => {
                    // println!("\nAccepted connection from {id}");
                    clients.insert(id, BufReader::new(stream));

                    println!("Active connections:");
                    for client_id in clients.keys() {
                        print!("  {client_id}");
                    }
                    print!("\nSelect ID, 'a' for all, or 'q' to quit:");
                    print!("\n> ");
                    io::stdout().flush()?;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break 'main String::from("q"),
            }

            match input_rx.try_recv() {
                Ok(line) => break line,
                Err(mpsc::TryRecvError::Empty) => {}
                Err(mpsc::TryRecvError::Disconnected) => break 'main String::from("q"),
            }
        };

        let selection = selection.trim();

        if selection == "q" {
            break;
        }

        'subMenu: loop {
            // For multi-cast - vector
            let selected_ids: Vec<u64> = if selection == "a" || selection == "all" {
                clients.keys().copied().collect()
            } else {
                // If only one id
                match selection.trim().parse::<u64>() {
                    Ok(id) if clients.contains_key(&id) => vec![id],
                    _ => {
                        println!("That client ID is not in the active list.");
                        break;
                    }
                }
            };

            // handle new command loop for selected
            let command = match handle_action_selection(&input_rx)? {
                Some(command) => command,
                None => break 'subMenu, // Back to the client list
            };

            let mut disconnected_ids = Vec::new();

            for id in selected_ids {
                if let Some(stream) = clients.get_mut(&id) {
                    if handle_selected_client(stream, &command)? {
                        println!("Client {id} disconnected.");
                        disconnected_ids.push(id);
                    }
                }
            }

            for id in disconnected_ids {
                clients.remove(&id);
            }
        }
    }

    fs::remove_file(path)?;
    Ok(())
}

// Needed for multi-cast to work - I can't be passing a specific stream to the options menu
// Just need to confirm valid result - and return chosen command
// Option is Rust's version of null values - either you have Some(command) or none(for back)
fn handle_action_selection(input_rx: &mpsc::Receiver<String>) -> io::Result<Option<ServerCommand>> {
    loop {
        // Adding new option for multi-cast
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

        let input = match input_rx.recv() {
            Ok(input) => input,
            Err(_) => return Ok(None),
        };

        let command = match input.trim() {
            "1" => ServerCommand::Ping,
            "2" => ServerCommand::ShowNotification {
                message: "Hello from the server!".to_owned(),
            },
            "3" => {
                // so from root menu - created assets and can use this environmental variable
                let path =
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/ed-head.png");

                ServerCommand::ShowImage {
                    filename: "ed-head.png".to_owned(),
                    data: fs::read(path)?,
                }
            }
            "4" => {
                // so from root menu - created assets and can use this environmental variable
                let path =
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets/sound.mp3");
                ServerCommand::PlaySound {
                    filename: "sound.mp3".to_owned(),
                    data: fs::read(path)?,
                }
            }
            "b" | "back" => return Ok(None),
            _ => {
                println!("Enter 1, 2, 3, 4, or b.");
                continue;
            }
        };
        return Ok(Some(command));
    }
}

fn handle_selected_client(
    stream: &mut BufReader<UnixStream>,
    command: &ServerCommand,
) -> io::Result<bool> {
    let mut line = serde_json::to_string(command)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    line.push('\n');

    if let Err(err) = stream.get_mut().write_all(line.as_bytes()) {
        eprintln!("Client disconnected: {err}");
        return Ok(true);
    }

    if let Err(err) = stream.get_mut().flush() {
        eprintln!("Client disconnected: {err}");
        return Ok(true);
    }

    let mut response = String::new();
    match stream.read_line(&mut response) {
        Ok(0) => return Ok(true), // EOF: client closed the connection
        Ok(_) => {}
        Err(err) => {
            eprintln!("Client disconnected: {err}");
            return Ok(true);
        }
    }

    match serde_json::from_str::<CommandResult>(response.trim()) {
        Ok(result) => println!("Client response: {result:?}"),
        Err(err) => eprintln!("Could not parse client response: {err}"),
    }

    Ok(false) // Client is still connected
}

