// Example 1 - basic connection, send
fn main() -> std::io::Result<()> {
    let response = domain_socket::run_client("/tmp/domain-socket.sock")?;
    print!("Server replied: {response}");
    Ok(())
}

