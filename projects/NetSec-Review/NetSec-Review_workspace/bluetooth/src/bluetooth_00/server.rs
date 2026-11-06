// See readme README_BLUETOOTH.md for program descriptions
use bluer::rfcomm::{Listener, SocketAddr};
use tokio::io::AsyncReadExt;

pub async fn run_server() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let address = adapter.address().await?;
    let listener = Listener::bind(SocketAddr::new(address, 1)).await?;
    println!("Listening on RFCOMM channel 1");

    loop {
        let (mut stream, _peer) = listener.accept().await?;
        // println!("Connected: {peer:?}");

        let mut buf = [0; 1024];
        loop {
            let n = stream.read(&mut buf).await?;
            if n == 0 {
                break; // Client disconnected
            }
            println!("Received: {}", String::from_utf8_lossy(&buf[..n]));
        }
    }
}
