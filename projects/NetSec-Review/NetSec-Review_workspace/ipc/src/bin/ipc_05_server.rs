// --- Domain socket (IPC) demo ---
// --- --- User Messenger ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_05::server::run_server;

fn main() -> std::io::Result<()> {
    // UnixDatagram - client and server operate off different socket paths
    run_server("/tmp/domain-socket-1.sock");
    Ok(())
}
