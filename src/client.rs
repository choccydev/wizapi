use anyhow::Error;
use std::{
    collections::HashMap,
    net::IpAddr,
    path::{Path, PathBuf},
};
use tokio::sync::Mutex;
use wizrpc::{Client as WizRpcClient, Method, Param, Request, Response};

use crate::model::{
    Client, CommonControl, CommonOperation, Controllable, DeviceClass, LampControl, LampOperation,
    Operation,
};

impl Client {
    // ...

    async fn perform(
        &mut self,
        controllable_name: &str,
        operation: Operation,
    ) -> Result<(), Error> {
        let mut controllables = self.controllables.lock().await;
        if let Some(controllable) = controllables.get_mut(controllable_name) {
            match controllable {
                Controllable::Device(device) => {
                    match operation {
                        Operation::Common(CommonOperation::TurnOn) => device.turn_on(),
                        Operation::Common(CommonOperation::TurnOff) => device.turn_off(),
                        Operation::Common(CommonOperation::Toggle) => device.toggle(),
                        Operation::Lamp(_) => {}
                    }
                    match device {
                        DeviceClass::Lamp(lamp) => match operation {
                            Operation::Lamp(LampOperation::SetIntensity(intensity)) => {
                                lamp.set_intensity(intensity)
                            }
                            Operation::Lamp(LampOperation::SetColor(color)) => {
                                lamp.set_color(color)
                            }
                            Operation::Lamp(LampOperation::GetState) => lamp.get_state(),
                            Operation::Lamp(LampOperation::GetIntensity) => lamp.get_intensity(),
                            Operation::Lamp(LampOperation::GetColor) => lamp.get_color(),

                            Operation::Common(_) => {}
                        },
                    }
                }
                Controllable::Group(group) => {
                    // TODO iterate over the group calling perform_operation on each device
                    unimplemented!()
                }
                Controllable::Zone(zone) => {
                    // TODO iterate over the zone calling perform_operation on each group
                    unimplemented!()
                }
            }
        }
        Ok(())
    }

    pub async fn new(config_path: &str) -> Result<Self, Error> {
        todo!()
        // let rpc_client = WizRpcClient::default().await?;
        // // Load configurations and populate devices HashMap
        // // ...
        // Ok(Self {
        //     rpc_client: Mutex::new(rpc_client),
        //     devices: Mutex::new(HashMap::new()),
        //     // ...
        // })
    }

    // Discover devices on the network
    pub async fn discover_devices(&self) -> Result<(), Error> {
        todo!()
    }

    // Add device to the config
    pub async fn add_device(&self, ip: IpAddr, name: &str) -> Result<(), Error> {
        todo!()
    }

    // Remove device from the config
    pub async fn remove_device(&self, name: &str) -> Result<(), Error> {
        todo!()
    }

    // Persist serialized config into a given location
    pub async fn persist_config(&self, location: PathBuf) -> Result<(), Error> {
        todo!()
    }

    // Load and deserialize config from the given location
    pub async fn load_config(&self, location: PathBuf) -> Result<(), Error> {
        todo!()
    }

    // Add group to the config
    pub async fn add_group(&self, name: &str, devices: Vec<String>) -> Result<(), Error> {
        todo!()
    }

    // Add element to group
    pub async fn add_to_group(&self, group: &str, device: String) -> Result<(), Error> {
        todo!()
    }

    // Remove element from group
    pub async fn remove_from_group(&self, group: &str, device: String) -> Result<(), Error> {
        todo!()
    }

    // Remove group from the config
    pub async fn remove_group(&self, name: &str) -> Result<(), Error> {
        todo!()
    }

    // Add zone to the config
    pub async fn add_zone(&self, name: &str, groups: Vec<String>) -> Result<(), Error> {
        todo!()
    }

    // Remove zone from the config
    pub async fn remove_zone(&self, name: &str) -> Result<(), Error> {
        todo!()
    }

    // Add element to group
    pub async fn add_to_zone(&self, zone: &str, group: String) -> Result<(), Error> {
        todo!()
    }

    // Remove element from group
    pub async fn remove_from_zone(&self, zone: &str, group: String) -> Result<(), Error> {
        todo!()
    }
}
