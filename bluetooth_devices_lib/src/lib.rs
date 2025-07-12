use windows::{
    core::{HSTRING, Result},
    Devices::{
        Bluetooth::{BluetoothAdapter, BluetoothDevice},
        Enumeration::{DeviceInformation, DeviceWatcher},
    },
    Foundation::TypedEventHandler,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use uuid::Uuid;
use windows::Devices::Radios::RadioState;

#[derive(Debug, Clone)]
pub enum DeviceType {
    Headset,
    Keyboard,
    Speaker,
    Other,
}


#[derive(Debug, Clone)]
pub struct BluetoothManager {

}

impl BluetoothManager {
    pub fn new() -> Self {
        Self {

        }
    }




    pub fn get_bluetooth_adapter(&mut self) -> Result<BluetoothAdapter> {
        let async_op = BluetoothAdapter::GetDefaultAsync()?;
        async_op.get()
    }
    pub fn toggle_bluetooth(&mut self ,enable: bool) -> Result<RadioState> {
        let adapter = self.get_bluetooth_adapter()?;
        let radio_async = adapter.GetRadioAsync()?;
        let radio = radio_async.get()?;

        let target_state = if enable {
            RadioState::On
        } else {
            RadioState::Off
        };

        let set_async = radio.SetStateAsync(target_state)?;
        set_async.get()?;
        Ok(radio.State()?)
    }
    pub fn get_bluetooth_state(&mut self) -> Result<RadioState> {
        let adapter = self.get_bluetooth_adapter()?;
        let radio_async = adapter.GetRadioAsync()?;
        let radio = radio_async.get()?;
        Ok(radio.State()?)
    }
    pub fn get_bluetooth_state_int(&mut self) -> Result<i8> {
        let f = self.get_bluetooth_state()?;
        match self.get_bluetooth_state()? {
            RadioState::On => {
                println!("Bluetooth включен, выключаем...");
                Ok(1)
            }
            RadioState::Off => {
                println!("Bluetooth выключен, включаем...");
                Ok(0)
            }
            _ => {
                println!("Неизвестное состояние Bluetooth");
                Ok(-1)
            },
        }
    }

    pub async fn test() -> bool {
        false
    }
    
    
    pub fn scan_devices( ) -> Result<Vec<String>> {
        let found_devices = Arc::new(Mutex::new(Vec::new()));
        let selector = BluetoothDevice::GetDeviceSelector()?;
        let watcher = DeviceInformation::CreateWatcherAqsFilter(&selector)?;

        // Store the token for cleanup
        let mut added_token = None;

        // Handle device discovery events
        let added_handler = {
            let devices = Arc::clone(&found_devices);
            TypedEventHandler::<DeviceWatcher, DeviceInformation>::new(
                move |_sender, info_ref| {
                    let info = info_ref.as_ref(); // Convert Ref to reference
                    if let (Ok(name), Ok(id)) = (info.expect("safas1").Name(), info.expect("safas2").Id()) {
                        let device_str = format!(
                            "{} ({})",
                            name.to_string_lossy(),
                            id.to_string_lossy()
                        );
                        devices.lock().unwrap().push(device_str);
                    }
                    Ok(())
                }
            )
        };

        // Register the event handler and store the token
        added_token = Some(watcher.Added(&added_handler)?);
        watcher.Start()?;


        tokio::time::sleep(Duration::from_secs(10));


        watcher.Stop()?;
        if let Some(token) = added_token {
            watcher.RemoveAdded(token)?;
        }

        // Extract the discovered devices
        let devices = Arc::try_unwrap(found_devices)
            .expect("Failed to unwrap Arc")
            .into_inner()
            .expect("Failed to get inner vector from Mutex");

        Ok(devices)
    }
}
