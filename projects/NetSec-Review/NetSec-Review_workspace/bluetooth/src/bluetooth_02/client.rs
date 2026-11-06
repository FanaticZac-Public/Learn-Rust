// See readme README_BLUETOOTH.md for program descriptions

use bluer::{
    l2cap::{Datagram, SocketAddr},
    Address, AddressType,
};
use std::error::Error;

const SERVER_PSM: u16 = 0x100B;

pub async fn run_client(
    server_address: &str,
) -> Result<(), Box<dyn Error>> {
    let address: Address = server_address.parse()?;

    // Dynamically allocate the CLIENT's local endpoint.
    let socket = Datagram::bind(SocketAddr::any_br_edr()).await?;

    println!(
        "CLIENT local socket: {:?}",
        socket.as_ref().local_addr()?
    );

    let server = SocketAddr::new(
        address,
        AddressType::BrEdr,
        SERVER_PSM,
    );

    println!("Sending to {server:?}");

        socket.connect(server).await?;

    println!("Datagram peer connected!");

    for seq in 0u32..100 {
        let message = format!("Bluetooth test packet {seq}");

        let mut packet = Vec::new();
        packet.extend_from_slice(&seq.to_le_bytes());
        packet.extend_from_slice(message.as_bytes());

        let sent = socket.send_to(&packet, server).await?;

        println!("Sent packet {seq}: {sent} bytes");

        tokio::time::sleep(
            std::time::Duration::from_millis(100)
        ).await;
    }

    println!("Finished sending.");

    Ok(())
}
