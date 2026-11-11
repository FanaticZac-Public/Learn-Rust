// See readme README_BLUETOOTH.md for program descriptions

use bluer::{AdapterEvent, Session};
use futures::StreamExt;

// Just checking out this discovery function
pub async fn scan() -> bluer::Result<()> {
    let session = Session::new().await?;
    let adapter = session.default_adapter().await?;

    adapter.set_powered(true).await?;
    println!("Scanning with adapter {}", adapter.name());

    let events = adapter.discover_devices().await?;
    futures::pin_mut!(events);

    while let Some(event) = events.next().await {
        if let AdapterEvent::DeviceAdded(address) = event {
            let device = adapter.device(address)?;
            let name = device.name().await?.unwrap_or_else(|| "(unnamed)".into());
            println!("{address}: {name}");
        }
    }

    Ok(())
}