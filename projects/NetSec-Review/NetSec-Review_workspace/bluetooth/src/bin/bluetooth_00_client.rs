// --- BLUETOOTH demo ---
// --- --- Basic ---
// --------------------------------
// See readme README_BLUETOOTH.md for program descriptions

use bluetooth::bluetooth_00::client::run_client;

// just created a little file to store my mac to add to gitignore (in lieu of .env)
// I didn't want to write a build.rs for this purpose
#[path = "../bluetooth_00/env.rs"]
mod env;

#[tokio::main]
async fn main() -> bluer::Result<()> {
   let server_address = env::SERVER_MAC_ADDRESS;

    // run_client("AA:BB:CC:DD:EE:FF").await
    run_client(&server_address).await
}