// See readme README_BLUETOOTH.md for program descriptions
use bluer::rfcomm::{Listener, SocketAddr};
use tokio::io::AsyncReadExt;
use aes_gcm::{Aes256Gcm, Key};
use crate::bluetooth_01::my_cryptor;

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let address = adapter.address().await?;
    let listener = Listener::bind(SocketAddr::new(address, 1)).await?;
    println!("Listening on RFCOMM channel 1");

    // my fake key for example
    let key_bytes = [7u8; 32];
    let key = Key::<Aes256Gcm>::try_from(&key_bytes[..])?;

    loop {
        let (mut stream, _peer) = listener.accept().await?;
        // println!("Connected: {peer:?}");

        // So in this we're sending one packet with byte length of data packet to be expected following
        // This is common in lower level sock communication so can set the buffers correctly without buffer overflow, expects a 4B len, then a whatever it says len for data.
        let mut len_bytes = [0u8; 4];
        stream.read_exact(&mut len_bytes).await?;

        let len = u32::from_be_bytes(len_bytes) as usize;

        if len > 64 * 1024 {
            return Err("message too large".into());
        }

        let mut encrypted = vec![0u8; len];
        stream.read_exact(&mut encrypted).await?;

        // for comparing in wireshark output
        println!("{:?}", &encrypted.iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>());

        let plaintext = my_cryptor::decrypt(&key, &encrypted)?;
        println!("Received: {plaintext}");
    }
    
}
