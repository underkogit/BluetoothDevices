use bluetooth_devices_lib::*;
use windows::{
    core::{ Result},

};


#[tokio::main]
async fn main() -> Result<()>{
    let mut bluetooth_manager = BluetoothManager::new();

    let status = bluetooth_manager.toggle_bluetooth(true);

    match status {
        Ok(_) => {
            let list_devices = bluetooth_manager.get_devices_all().await?;
            for device in list_devices {
                println!("{}" , device)
            }

        }
        Err(_) => {
            println!("Failed to toggle bluetooth");
        }
    }
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    Ok(())
}
