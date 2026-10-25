fn main() -> std::io::Result<()> {
    domain_socket::run_server("/tmp/domain-socket.sock")
}