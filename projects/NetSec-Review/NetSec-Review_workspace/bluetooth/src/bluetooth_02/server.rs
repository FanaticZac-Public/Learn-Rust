// See readme README_BLUETOOTH.md for program descriptions
use bluer::{
    l2cap::{Datagram, SocketAddr},
    Address, AddressType,
};
use std::error::Error;

// I kept getting "Address already in use" for 0x1001 going up by 2 up to that PSM"
const PSM: u16 = 0x100B;

pub async fn run_server(
    server_address: &str,
) -> Result<(), Box<dyn Error>> {

    let address: Address = server_address.parse()?;
    let local = SocketAddr::new(address, AddressType::BrEdr, PSM);
    // let local = SocketAddr::any_br_edr();
    let socket = Datagram::bind(local).await?;
    let actual = socket.as_ref().local_addr()?;
//    eprintln!("SERVER: listening at {actual:?}");
      eprintln!("SERVER: listening...");

    let mut packet = [0u8; 1024];
    let mut received = 0u32;

    loop {
        let (n, sender) = socket.recv_from(&mut packet).await?;

        if n < 4 {
            println!("Invalid packet: only {n} bytes");
            continue;
        }

        let seq = u32::from_le_bytes(
            packet[..4].try_into()?
        );

        let message = String::from_utf8_lossy(
            &packet[4..n]
        );

        // println!(
        //     "Received #{seq} from {sender:?}: {message}"
        // );
              println!(
            "Received #{seq} from {0:?}, {1:?}: {message}", sender.addr_type, sender.psm
        );

        received += 1;
    }
}