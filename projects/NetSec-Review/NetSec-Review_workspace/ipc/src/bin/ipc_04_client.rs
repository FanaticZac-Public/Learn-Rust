// --- Domain socket (IPC) demo --- 
// --- --- User Messenger ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_04::client::run_client;

fn main() -> std::io::Result<()> {
    run_client("/tmp/domain-socket.sock")?;
    Ok(())
}

