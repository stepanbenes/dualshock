use BlueZ::*;

fn main() {
    // Create a new session. This establishes the D-Bus connection to talk to BlueZ. In this case we
    // ignore the join handle, as we don't intend to run indefinitely.
    let (_, session) = BluetoothSession::new().await?;

    // Start scanning for Bluetooth devices, and wait a few seconds for some to be discovered.
    session.start_discovery().await?;
    time::sleep(Duration::from_secs(5)).await;
    session.stop_discovery().await?;

    // Get a list of devices which are currently known.
    let devices = session.get_devices().await?;

    // Find the device we care about.
    let device = devices
        .into_iter()
        .find(|device| device.name.as_deref() == Some("My device"))
        .unwrap();

    // Connect to it.
    session.connect(&device.id).await?;

    // Look up a GATT service and characteristic by short UUIDs.
    let service = session
        .get_service_by_uuid(&device.id, uuid_from_u16(0x1234))
        .await?;
    let characteristic = session
        .get_characteristic_by_uuid(&service.id, uuid_from_u32(0x1235))
        .await?;

    // Read the value of the characteristic and write a new value.
    println!(
        "Value: {:?}",
        session
            .read_characteristic_value(&characteristic.id)
            .await?
    );
    session
        .write_characteristic_value(&characteristic.id, vec![1, 2, 3])
        .await?;

    // Subscribe to notifications on the characteristic and print them out.
    let mut events = session
        .characteristic_event_stream(&characteristic.id)
        .await?;
    session.start_notify(&characteristic.id).await?;
    while let Some(event) = events.next().await {
        if let BluetoothEvent::Characteristic {
            id,
            event: CharacteristicEvent::Value { value },
        } = event
        {
            println!("Update from {}: {:?}", id, value);
        }
    }
}
