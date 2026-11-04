// See readme README_BLUETOOTH.md for program descriptions

use bluer::{
    rfcomm::{SocketAddr, Stream},
    Address,
};
use tokio::io::AsyncWriteExt;
use crate::bluetooth_01::my_cryptor;
use aes_gcm::{Aes256Gcm, Key};

pub async fn run_client(server_address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let address: Address = server_address
        .parse()
        .expect("invalid Bluetooth address");

    let mut stream = Stream::connect(SocketAddr::new(address, 1)).await?;

    // my fake key
    let key_bytes = [7u8; 32];
    let key = Key::<Aes256Gcm>::try_from(&key_bytes[..])?;

    let message = my_cryptor::encrypt(&key, "Hello from the client!\n")?;
    let len = u32::try_from(message.len())?;

    stream.write_all(&len.to_be_bytes()).await?;
    stream.write_all(&message).await?;


    Ok(())
}