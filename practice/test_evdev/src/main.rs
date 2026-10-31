use evdev::{Device, EventSummary, KeyCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Replace this with the event node for your test device.
    let mut device = Device::open("/dev/input/event2")?;
    println!("Reading from: {}", device.name().unwrap_or("unnamed device"));

    loop {
        for event in device.fetch_events()? {
            if let EventSummary::Key(_, KeyCode::KEY_A, 1) = event.destructure() {
                println!("A pressed");
            }
        }
    }
}
