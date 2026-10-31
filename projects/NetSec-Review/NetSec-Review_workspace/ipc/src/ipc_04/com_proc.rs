use serde::{Deserialize, Serialize};

// Let's add some meat to these potatoes - aka exchange string for proper data types

// Vec<u8> matches fs::read()
#[derive(Debug, Serialize, Deserialize)]
pub enum ServerCommand {
    Ping,
    ShowNotification { message: String },
    ShowImage { filename: String, data: Vec<u8> },
    PlaySound { filename: String, data: Vec<u8> }, 
    // AlertShutdown
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandResult {
    Pong,
    NotificationShown,
    ImageShown,
    SoundStarted,
}

