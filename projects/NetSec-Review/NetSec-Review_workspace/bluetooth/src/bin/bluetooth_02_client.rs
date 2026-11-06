// --- BLUETOOTH demo ---
// --- --- Basic ---
// --------------------------------
// See readme README_BLUETOOTH.md for program descriptions

use bluetooth::bluetooth_02::client::run_client;

// just created a little file to store my mac to add to gitignore (in lieu of .env)
// I didn't want to write a build.rs for this purpose
#[path = "../env.rs"]
#[allow(dead_code)]  // gets rid of warning for odd "unused" import of env outside the library compilation
mod env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {
   let server_address = env::SERVER_MAC_ADDRESS;

    // run_client("AA:BB:CC:DD:EE:FF").await
    run_client(&server_address).await
}