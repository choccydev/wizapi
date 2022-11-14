use super::device_model::DeviceDescriptor;
use super::network_model::{Params, ParamsFilter};
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

#[derive(Error, Debug)]
pub enum QueryError {
    #[error("Not all keys required by the method are present on the parameters given.\n Required Parameters: {filter:#?}\n Given parameters: {params:#?}")]
    FilterError {
        params: Params,
        filter: ParamsFilter,
    },
}
