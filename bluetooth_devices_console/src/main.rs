use bluetooth_devices_lib::*;

#[tokio::main]
async fn main() {
    let mut bluetooth_manager = BluetoothManager::new();

    let status = bluetooth_manager.toggle_bluetooth(true);
    match status {
        Ok(_) => {

            //let listDevices = bluetooth_manager(10).await;
        }
        Err(_) => {
            println!("Failed to toggle bluetooth");
        }
    }
    std::thread::sleep(std::time::Duration::from_secs(5));
}
