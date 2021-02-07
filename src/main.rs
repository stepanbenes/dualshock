//! Example to scan for a short time and then list all known devices.

use bluez_async::{BluetoothSession, DiscoveryFilter};
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::time;

const SCAN_DURATION: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //pretty_env_logger::init();

    let (_, session) = BluetoothSession::new().await?;

    // Start scanning for Bluetooth devices, and wait a while for some to be discovered.
    session.start_discovery().await?;
    time::sleep(SCAN_DURATION).await;
    session.stop_discovery().await?;

    // Get the list of all devices which BlueZ knows about.
    let devices = session.get_devices().await?;
    println!("Devices: {:#?}", devices);

    //! Example to log Bluetooth events, including duplicate manufacturer-specific advertisement data.
    //let (_, session) = BluetoothSession::new().await?;
    
    let mut events = session.event_stream().await?;
    session.start_discovery_with_filter(&DiscoveryFilter {
                duplicate_data: Some(true),
                ..DiscoveryFilter::default()
            })
            .await?;

    println!("Events:");
    while let Some(event) = events.next().await {
        println!("{:?}", event);
    }

    Ok(())
}
