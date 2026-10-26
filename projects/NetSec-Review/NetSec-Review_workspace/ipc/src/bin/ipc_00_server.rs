// --- Domain socket (IPC) demo --- 
// --- --- Basic ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_00::server::run_server;

fn main() -> std::io::Result<()> {
    run_server("/tmp/domain-socket.sock")
}

