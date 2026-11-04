// --- BLUETOOTH demo --- 
// --- --- Basic ---
// --------------------------------
// See readme README_BLUETOOTH.md for program descriptions

use bluetooth::bluetooth_01::server::run_server;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_server().await
}