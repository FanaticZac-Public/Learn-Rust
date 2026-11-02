// See readme README_BLUETOOTH.md for program descriptions

use bluer::{
    rfcomm::{SocketAddr, Stream},
    Address,
};
use tokio::io::AsyncWriteExt;

pub async fn run_client(server_address: &str) -> bluer::Result<()> {
    let address: Address = server_address
        .parse()
        .expect("invalid Bluetooth address");

    let mut stream = Stream::connect(SocketAddr::new(address, 1)).await?;
    stream.write_all(b"Hello from the client!\n").await?;
    Ok(())
}