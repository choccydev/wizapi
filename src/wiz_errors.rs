use super::model::DeviceDescriptor;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SceneError {
    #[error("Bad scene ID given. Expected 1-32 or 1000, received {0}")]
    IDError(u32),
    #[error("Bad scene name given. Given {0}")]
    NameError(String),
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error(transparent)]
    Parse(ParseError),
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unable to determine the device type for the described device.\n Descriptor: {0:#?}")]
    UndefinedType(DeviceDescriptor),
    #[error("Failed to find a expected color temperature space in the given descriptor.\n Descriptor: {0:#?}")]
    ColorTemp(DeviceDescriptor),
}
