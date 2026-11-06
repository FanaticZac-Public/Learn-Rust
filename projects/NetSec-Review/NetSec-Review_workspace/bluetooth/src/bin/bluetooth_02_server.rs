// --- BLUETOOTH demo --- 
// --- --- Basic ---
// --------------------------------
// See readme README_BLUETOOTH.md for program descriptions

use bluetooth::bluetooth_02::server::run_server;

#[path = "../env.rs"]
#[allow(dead_code)] // gets rid of warning for odd "unused" import of env outside the library compilation
mod env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {  
    let server_address = env::SERVER_MAC_ADDRESS;

    // run_server("AA:BB:CC:DD:EE:FF").await
    run_server(&server_address).await


}