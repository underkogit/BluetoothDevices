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
use std::sync::LockResult;
use uuid::Uuid;
use windows::Devices::Enumeration::{DeviceClass, DeviceInformationCollection};
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

    pub fn test() -> bool {
        false
    }

    // pub async fn get_devices_all(&mut self) -> Result<DeviceInformationCollection> {
    //     // Получаем список всех устройств
    //     let list_result = DeviceInformation::FindAllAsyncDeviceClass(DeviceClass::AudioRender);
    // 
    //     // Обрабатываем результат
    //     match list_result {
    //         Ok(devices) => {
    //             let mut found_devices:Vec<DeviceInformation> = Vec::new();
    //             let devices = devices.get().unwrap();
    // 
    //             for device in devices {
    //                 // Предположим, что у устройства есть метод Name() и Id()
    //                 if let (Ok(name), Ok(id)) = (device.Name(), device.Id()) {
    //                     // let _name = name.to_string_lossy();
    //                     // let _id = id.to_string_lossy();
    //                     // let device_str = format!("{} ({})",_name, _id);
    // 
    //                     
    // 
    //                     found_devices.push(device_str);
    //                 }
    //             }
    // 
    //             Ok(devices) // Возвращаем список найденных устройств
    //         },
    //         Err(e) => {
    //             // Обработка ошибки
    //             eprintln!("Ошибка при поиске устройств: {:?}", e);
    //             Err(e) // Возвращаем ошибку
    //         }
    //     }
    // }


    pub async fn get_scan_devices(&mut self, sleep:u64) -> Result<Vec<String>> {
        let found_devices = Arc::new(Mutex::new(Vec::new()));
        let selector = BluetoothDevice::GetDeviceSelector()?;





        //println!("{}" , selector.to_string_lossy());
        let watcher = DeviceInformation::CreateWatcherAqsFilter(&selector)?;

        // Store the token for cleanup
        let mut added_token = None;

        // Handle device discovery events
        let added_handler = {
            let devices = Arc::clone(&found_devices);
            TypedEventHandler::<DeviceWatcher, DeviceInformation>::new(
                move |_sender, info_ref| {
                    let info = info_ref.as_ref().unwrap(); // Convert Ref to reference

                    if let
                        (Ok(name), Ok(id) , Ok(is_enabled) , Ok(is_default)) =
                        (info.Name(), info.Id() , info.IsEnabled() , info.IsDefault()) {

                        let device_id = id; // Convert ID to string



                         let ddevice = windows::Devices::Bluetooth::BluetoothDeviceId::FromId(&"Bluetooth#Bluetooth8c:68:8b:e0:6b:e2-55:bf:9e:4f:b8:f0".into())?;
                        // println!("{:?}", ddevice);

                       // let ddevice = windows::Devices::Bluetooth::   BluetoothDeviceId::FromId(&(info.Name().unwrap()));

                        //windows::Devices::Bluetooth::BluetoothConnectionStatus
                        let device_str = format!(
                            "{} [{}][{}] ({})",
                            name.to_string_lossy(),
                            is_enabled,
                            is_default,
                            device_id.to_string_lossy()
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

        // Wait for a while to discover devices
        tokio::time::sleep(Duration::from_secs(sleep)).await;

        watcher.Stop()?;
        if let Some(token) = added_token {
            watcher.RemoveAdded(token)?;
        }

        // Получите вектор устройств
        let devices = found_devices.lock().unwrap(); // Исправлено: используйте `unwrap` для получения доступа к вектору

        // Возвращаем вектор устройств
        Ok(devices.clone()) // Клонируем вектор, чтобы вернуть его
    }
}
