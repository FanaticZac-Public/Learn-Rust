// --- Domain socket (IPC) demo --- 
// --- --- User Messenger ---
// --------------------------------
// See readme README_IPC.md for program descriptions

use ipc::ipc_01::server::run_server;

fn main() -> std::io::Result<()> {
    set_linux_thread_name("my-server")?;
    run_server("/tmp/domain-socket.sock")
}

