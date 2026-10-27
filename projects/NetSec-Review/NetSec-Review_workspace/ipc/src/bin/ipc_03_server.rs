// --- Domain socket (IPC) demo ---
// --- --- User Messenger ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_03::server::run_server;

fn main() -> std::io::Result<()> {
    run_server("/tmp/domain-socket.sock")
}
