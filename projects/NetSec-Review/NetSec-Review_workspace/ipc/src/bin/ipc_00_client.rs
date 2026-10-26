// --- Domain socket (IPC) demo --- 
// --- --- Basic ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_00::client::run_client;

fn main() -> std::io::Result<()> {
    let response = run_client("/tmp/domain-socket.sock")?;
    print!("Server replied: {response}");
    Ok(())
}

