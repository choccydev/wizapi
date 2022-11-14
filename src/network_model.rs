use super::wiz_errors::QueryError;
use super::wiz_errors::SceneError;
use anyhow::Error;
use bytemuck::{cast, try_cast};
use chrono::Duration;
use http::Uri;
use lazy_static::lazy_static;
use macaddr::MacAddr6;
use num::{FromPrimitive, Integer, One, Zero};
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::NumOps;
use optional_struct::OptionalStruct;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Number, Value};
use std::{any::Any, collections::HashMap};
use uuid::Uuid;

lazy_static! {
    pub static ref RESPONSE_PORT: u16 = 38899;
    pub static ref LISTENING_PORT: u16 = 38900;
    pub static ref DEFAULT_NETWORK_CONFIG: NetworkConfig = NetworkConfig {
        timeout: Duration::seconds(13),
        max_sent_datagrams: 6,
        first_send_interval: Duration::milliseconds(750),
        max_backoff: 3,
        keepalive: Duration::milliseconds(20),
        port: *RESPONSE_PORT,
    };
    // TODO add this remaining stuff according to reverse engineering andf the pywiz stuff
   //  pub static ref METHODS: Methods = Methods {
        // update: Method::new("getPilot", false, None, None),
        // get_power: Method::new("getPower", true, None, None), // ? Unknown
        // get_system: Method::new("getSystemConfig", false, None, None),
        // get_model: Method { name: "getModelConfig", experimental: false, params: None, extra_members: None },
        // get_user: Method { name: "getUserConfig", experimental: false, params: None, extra_members: None },
        // set_legacy: "setPilot",
        // set: "setState",
        // reboot: "reboot",
        // reset: "reset"
    // };
}

#[derive(Debug, Clone)]
pub struct NetworkConfig {
    pub timeout: Duration,
    pub max_sent_datagrams: u8,
    pub first_send_interval: Duration,
    pub max_backoff: u8,
    pub keepalive: Duration,
    pub port: u16,
}

pub struct MethodQuery {
    pub method: Method,
    params: Option<Params>,
}

impl MethodQuery {
    pub fn new(method: Method, params: Option<Params>) -> Result<Self, QueryError> {
        Ok(MethodQuery {
            method: method.clone(),
            params: if let Some(parameters) = params {
                match method.filter {
                    None => None,
                    Some(filter) => Some(parameters.filter(filter)?),
                }
            } else {
                None
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct Method {
    name: &'static str,
    experimental: bool,
    filter: Option<ParamsFilter>,
    extra_members: Option<Value>,
}

impl Method {
    pub fn new(
        name: &'static str,
        experimental: bool,
        filter: Option<ParamsFilter>,
        extra_members: Option<Value>,
    ) -> Self {
        let extra = Map::new();

        Method {
            name: name,
            experimental: experimental,
            filter: filter,
            extra_members: extra_members,
        }
    }
    pub fn params(mut self: Self, params: OptionalParams) {}
}

#[derive(Debug, Clone)]
pub struct Methods {
    pub get: GetMethods,
    pub set: SetMethods,
    pub status: StatusMethods,
    pub update: Method,
}

#[derive(Debug, Clone)]
pub struct GetMethods {
    pub get_power: Method,
    pub get_system: Method,
    pub get_model: Method,
    pub get_user: Method,
    pub get_direct: Method,
    pub get_intensity: Method,
    pub get_hue: Method,
    pub get_speed: Method,
    pub get_scene: Method,
}

#[derive(Debug, Clone)]
pub struct SetMethods {
    pub set_legacy_any: Method,
    pub set_any: Method,
    pub set_intensity: Method,
    pub set_hue: Method,
    pub set_speed: Method,
    pub set_scene: Method,
}

#[derive(Debug, Clone)]
pub struct StatusMethods {
    pub reboot: Method,
    pub reset: Method,
    pub on: Method,
    pub off: Method,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamsFilter {
    pub state: bool,
    pub speed: bool,
    pub ratio: bool,
    pub scene: bool,
    pub brightness: bool,
}

impl ParamsFilter {
    fn to_map(self: Self) -> HashMap<String, bool> {
        serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap()
    }
}

#[derive(Debug, Clone, OptionalStruct, Serialize, Deserialize)]
#[optional_derive(Debug, Clone, Serialize, Deserialize)]
pub struct Params {
    pub state: Option<bool>,
    pub speed: Option<Number>,
    pub ratio: Option<Number>,
    pub scene: Option<Number>,
    pub brightness: Option<Number>,
}

impl Params {
    pub fn new() -> Self {
        Params {
            state: None,
            speed: None,
            ratio: None,
            scene: None,
            brightness: None,
        }
    }

    fn to_map(self: Self) -> HashMap<String, Value> {
        // TODO Add error handling
        serde_json::from_value(serde_json::to_value(self).unwrap()).unwrap()
    }

    fn from_map(map: HashMap<String, Value>) -> OptionalParams {
        // TODO Add error handling
        serde_json::from_value(serde_json::to_value(map).unwrap()).unwrap()
    }

    pub fn patch(mut self: Self, patch: OptionalParams) {
        self.apply_options(patch)
    }

    //? This function is sus, gotta test it well
    pub fn filter(self: Self, filter: ParamsFilter) -> Result<Self, QueryError> {
        let self_bind = self.clone();
        let filter_bind = filter.clone();
        let mut new_params = Self::new();
        let mut new_params_map: HashMap<String, Value> = HashMap::new();

        let self_map = self.to_map();
        let filter_map = filter.to_map();

        for (filter_param, is_expected) in filter_map {
            if is_expected {
                let value = self_map
                    .get(&filter_param)
                    .ok_or(QueryError::FilterError {
                        params: self_bind.clone(),
                        filter: filter_bind.clone(),
                    })?
                    .clone();
                new_params_map.insert(filter_param, value);
            }
        }

        new_params.apply_options(Self::from_map(new_params_map));

        Ok(new_params)
    }
}
