// --- Domain socket (IPC) demo ---
// --- --- User Messenger ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_05::client::run_client;

fn main() -> std::io::Result<()> {
    // UnixDatagram - client and server operate off different socket paths
    run_client("/tmp/domain-socket-2.sock", "/tmp/domain-socket-1.sock")?;
    Ok(())
}
