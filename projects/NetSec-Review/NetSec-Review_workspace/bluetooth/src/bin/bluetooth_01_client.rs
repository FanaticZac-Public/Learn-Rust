// --- BLUETOOTH demo ---
// --- --- Basic ---
// --------------------------------
// See readme README_BLUETOOTH.md for program descriptions

use bluetooth::bluetooth_01::client::run_client;

// just created a little file to store my mac to add to gitignore (in lieu of .env)
// I didn't want to write a build.rs for this purpose
#[path = "../env.rs"]
mod env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
   let server_address = env::SERVER_MAC_ADDRESS;

    // run_client("AA:BB:CC:DD:EE:FF").await
    run_client(&server_address).await
}