// --- Domain socket (IPC) demo --- 
// --- --- User Messenger ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_02::client::run_client;

fn main() -> std::io::Result<()> {
    let response = run_client("/tmp/domain-socket.sock")?;
    Ok(())
}

