// --- BLUETOOTH demo --- 
// --- --- Basic ---
// --------------------------------
// See readme README_BLUETOOTH.md for program descriptions

use bluetooth::bluetooth_00::server::run_server;

#[tokio::main]
async fn main() -> bluer::Result<()> {
    run_server().await
}