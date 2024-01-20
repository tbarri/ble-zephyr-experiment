use bluest::{Adapter, AdvertisingDevice};
use bluest::Uuid;
use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    for i in 0..100 {
        println!("****************************************");
        println!("Starting iteration loop: {i}");
        println!("****************************************");
        let hr_service_characteristic = Uuid::parse_str("0000180D-0000-1000-8000-00805f9b34fb").expect("Failed to parse H/B characteristic");
        let hr_measurement_characteristic = Uuid::parse_str("00002A37-0000-1000-8000-00805f9b34fb").expect("Failed to parse hr_measurement_characteristic characteristic");
        println!("Step 0: Characteristics of interest: {:?}", vec![hr_service_characteristic, hr_measurement_characteristic]);

        println!("Step 1: Initialise a bluetooth adaptor");
        let adapter = Adapter::default().await.expect("Failed to access a Bluetooth adaptor");
        adapter.wait_available().await.expect("Failed waiting on the Bluetooth adaptor to become available");

        println!("Step 2: Scan for the device with characteristic: {}", hr_service_characteristic);
        let service_filter = vec![hr_service_characteristic];
        let mut scan = adapter.scan(&service_filter).await.expect("Failed to scan for advertising packet");
        let peripheral: AdvertisingDevice = scan.next().await.unwrap();
        let peripheral = peripheral.device;

        println!("Step 3: Connect to the device");
        adapter.connect_device(&peripheral).await.expect("Failed to connect to peripheral");

        println!("Step 4: Attempt to discover hr_measurement_characteristic: {hr_measurement_characteristic}");
        for service in peripheral.services().await.expect("Failed to parse services") {
            for characteristic in service.characteristics().await.expect("Failed to parse characteristics") {
                if characteristic.uuid() == hr_measurement_characteristic {
                    println!("Step 5: Discovered characteristic: {}", hr_measurement_characteristic);
                    let _ = characteristic.notify().await.expect("Failed to enable notifications");
                    println!("Step 6: Enable notifications on characteristic: {}", hr_measurement_characteristic);
                }
            }
        }

        println!("Step 7: Disconnect from the device\n");
        adapter.disconnect_device(&peripheral).await.expect("Failed to disconnect from peripheral");
    }
}
