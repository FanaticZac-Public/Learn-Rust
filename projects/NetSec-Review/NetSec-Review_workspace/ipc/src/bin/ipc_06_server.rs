// --- Domain socket (IPC) demo ---
// --- --- User Messenger ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_06::server::run_server;

fn main() -> std::io::Result<()> {
    // Using abstract address - no file! what!?
    // Everything not-a-file!? You lied to me professor!
    let _ = run_server();
    Ok(())
}
