use super::model::{ColorTempSpace, DeviceDescriptor};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SceneIDError {
    pub given_id: u32,
    pub details: String,
}

impl fmt::Display for SceneIDError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Given Scene ID {}. Error: {}",
            self.given_id, self.details
        )
    }
}

impl Error for SceneIDError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Clone)]
pub struct SceneNameError {
    pub given_name: String,
    pub details: String,
}

impl fmt::Display for SceneNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Given Scene Name {}. Error: {}",
            self.given_name, self.details
        )
    }
}

impl Error for SceneNameError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Clone)]
pub struct DeviceTypeParseError {
    pub data: DeviceDescriptor,
    pub details: String,
}

impl fmt::Display for DeviceTypeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Device type could not be determined from the given descriptor: {:#?}\n {}",
            self.data, self.details
        )
    }
}

impl Error for DeviceTypeParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug, Clone)]
pub struct DeviceColorTempParseError {
    pub data: DeviceDescriptor,
    pub details: String,
}

impl fmt::Display for DeviceColorTempParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Failed to find a color temperature space in the given descriptor: {:#?}\n {}",
            self.data, self.details
        )
    }
}

impl Error for DeviceColorTempParseError {
    fn description(&self) -> &str {
        &self.details
    }
}
