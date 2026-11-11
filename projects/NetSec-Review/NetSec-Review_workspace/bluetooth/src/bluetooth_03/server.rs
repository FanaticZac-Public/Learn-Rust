// See readme README_BLUETOOTH.md for program descriptions
use bluer::{
    l2cap::{SeqPacketListener, SocketAddr},
    adv::Advertisement,
};
use std::{error::Error, time::{Duration}};
use tokio::time::timeout;

const PSM: u16 = 0x0081; 
const IO_TIMEOUT: Duration = Duration::from_secs(5);

pub async fn run_server(
    _server_address: &str,
) -> Result<(), Box<dyn Error>> {
   let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let address = adapter.address().await?;
    let address_type = adapter.address_type().await?;

    // This was the secret sauce (to resolve my non-connection issue)
    // ON LE (Low Energy) instead of classic connection 
    // you must advertise the service
    let _advertisement = adapter
        .advertise(Advertisement {
            discoverable: Some(true),
            local_name: Some("l2cap_server".to_string()),
            ..Default::default()
        })
        .await?;

    let local = SocketAddr::new(address, address_type, PSM);
    let listener = SeqPacketListener::bind(local).await?;

    // println!("Advertising on {address} ({address_type})");
    // println!("Listening on {:?}", listener.as_ref().local_addr()?);
    println!("I'm listeninggggg....");

    let (peer, remote) = listener.accept().await?;
    println!("Accepted L2CAP peer: {remote:?}");

    let mut buf = vec![0u8; 4096];

    loop {
        let n = match timeout(IO_TIMEOUT, peer.recv(&mut buf)).await {
            Ok(result) => result?,
            Err(_) => {
                eprintln!("No packet received within {IO_TIMEOUT:?}; waiting again");
                continue;
            }
        };

        if n == 0 {
            println!("Peer closed the channel");
            break;
        }

        println!("Got on whwhahwahwa...!!!!");
        peer.send(&buf[..n]).await?;
    }

    Ok(())
}

