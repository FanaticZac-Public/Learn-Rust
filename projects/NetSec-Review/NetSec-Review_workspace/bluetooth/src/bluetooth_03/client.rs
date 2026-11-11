// See readme README_BLUETOOTH.md for program descriptions
use bluer::{
    l2cap::{SeqPacket, SocketAddr},
    AddressType,
};

use std::{error::Error, time::{Duration, Instant, SystemTime, UNIX_EPOCH}};
use tokio::time::timeout;

const PSM: u16 = 0x0081; 
const PROBES: u64 = 50;
const GAP: Duration = Duration::from_millis(20);
const IO_TIMEOUT: Duration = Duration::from_secs(5);
const PACKET_LEN: usize = 32;

pub async fn run_client(
    server_address: &str,
) -> Result<(), Box<dyn Error>> {
 let remote = SocketAddr::new(server_address.parse()?, AddressType::LePublic, PSM);
    let sock = SeqPacket::connect(remote).await?;

    // println!("Local: {:?}", sock.as_ref().local_addr()?);
    // println!("Peer:  {:?}", sock.peer_addr()?);
    println!("Send MTU: {:?}", sock.as_ref().send_mtu());
    println!("Recv MTU: {:?}", sock.as_ref().recv_mtu());

    // Round trip time seconds
    let mut rtts = Vec::<u128>::with_capacity(PROBES as usize);
    let mut buf = vec![0u8; 4096];

    for seq in 0..PROBES {
        let mut packet = [0u8; PACKET_LEN];
        packet[0..8].copy_from_slice(&seq.to_le_bytes());
        packet[8..24].copy_from_slice(&(wall_us() as u128).to_le_bytes());
        // Remaining bytes are reserved for payload/pattern data.
        let sent_at = Instant::now();
        let send_unix_us = wall_us();

        sock.send(&packet).await?;

        let n = timeout(IO_TIMEOUT, sock.recv(&mut buf)).await??;

        // micro seconds
        let recv_unix_us = wall_us(); 
        let rtt_us = sent_at.elapsed().as_micros();

        if n != PACKET_LEN || buf[..8] != seq.to_le_bytes() {
            return Err(format!("bad echo for probe {seq}: {n} bytes").into());
        }

        rtts.push(rtt_us);

        println!("seq:{seq}-client_send_unix_us:{send_unix_us}-client_recv_unix_us:{recv_unix_us}-rtt_us:{rtt_us},bytes:{n}");

        tokio::time::sleep(GAP).await;
    }

    // Totals
    rtts.sort_unstable();
    let min = rtts[0];
    let max = *rtts.last().unwrap();
    let avg = rtts.iter().sum::<u128>() as f64 / rtts.len() as f64;
    let median = rtts[rtts.len() / 2];
    let p95 = rtts[(rtts.len() * 95 / 100).min(rtts.len() - 1)];
    let jitter = if rtts.len() > 1 {
        rtts.windows(2).map(|w| w[0].abs_diff(w[1])).sum::<u128>() as f64
            / (rtts.len() - 1) as f64
    } else { 0.0 };

    println!(
        "summary: samples={}, min={}us, max={}us, avg={:.2}us, median={}us, p95={}us, mean_abs_adjacent_rtt_change={:.2}us",
        rtts.len(), min, max, avg, median, p95, jitter
    );
    Ok(())
}

fn wall_us() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .unwrap_or_default().as_micros()
}