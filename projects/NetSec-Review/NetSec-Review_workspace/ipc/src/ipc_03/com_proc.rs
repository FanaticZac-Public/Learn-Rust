use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerCommand {
    Ping,
    ShowNotification { message: String },
    ShowImage { path: String },
    PlaySound { path: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandResult {
    Pong,
    NotificationShown,
    ImageShown,
    SoundStarted,
}

