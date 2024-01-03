use std::collections::HashMap;
use tokio::sync::Mutex;
use wizrpc::{Client as WizRpcClient, Method, Param, Request, Response};

pub enum DeviceClass {
    Lamp(Lamp),
    // Other device types can be added here in the future
}
pub struct Lamp {
    pub mac: String,
    pub ip: String,
    pub model: String,
    pub capabilities: Capabilities,
    pub state: LampState,
    pub name: String,
}

pub struct Capabilities {
    // Define various capabilities like dimmable, color range, etc.
}

pub enum PowerState {
    On,
    Off,
    // Other states can be added here
}

pub struct LampState {
    pub color: Color,
    pub intensity: u8,
    pub power_state: PowerState,
}

pub trait CommonControl {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
}

pub trait LampControl: CommonControl {
    fn set_intensity(&mut self, intensity: u8);
    fn set_color(&mut self, color: Color);
    fn get_state(&mut self);
    fn get_intensity(&mut self);
    fn get_color(&mut self);
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub type DeviceGroup = Vec<DeviceClass>;
pub type DeviceZone = HashMap<String, DeviceGroup>;

pub enum Controllable {
    Group(DeviceGroup),
    Device(DeviceClass),
    Zone(DeviceZone),
}

pub struct Client {
    pub rpc_client: Mutex<WizRpcClient>, // WizRpcClient wrapped in Mutex for thread safety
    pub devices: Mutex<Vec<String>>,
    pub groups: Mutex<HashMap<String, Vec<String>>>,
    pub zones: Mutex<HashMap<String, Vec<String>>>,
    pub controllables: Mutex<HashMap<String, Controllable>>,
}

pub enum CommonOperation {
    TurnOn,
    TurnOff,
    Toggle,
}

pub enum LampOperation {
    SetIntensity(u8),
    SetColor(Color),
    GetState,
    GetIntensity,
    GetColor,
    // Other operations
}

pub enum Operation {
    Common(CommonOperation),
    Lamp(LampOperation),
}
