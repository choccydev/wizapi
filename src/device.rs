use super::device_model::{
    Bulb, ColorTemp, ColorTempSpace, ColorType, DeviceConfig, DeviceDefinition, DeviceDescriptor,
    DeviceFeatures, DeviceState, DeviceType, Intensity, OptionalDeviceConfig,
    OptionalDeviceDescriptor, OptionalDeviceFeatures, Rgb, WhiteStaticType, WhiteTunableType,
    WhiteType, WhiteVariableType, DEVICE_OPTS, KNOWN_TYPE_IDS,
};
use super::network_model::{NetworkConfig, DEFAULT_NETWORK_CONFIG};
use super::wiz_errors::{DeviceError, ParseError};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Device {
    Bulb(Bulb),
    Socket(DeviceDefinition),
}

impl Device {
    pub fn new(
        device_type: DeviceType,
        features: Option<DeviceFeatures>,
        descriptor: Option<DeviceDescriptor>,
        config: Option<DeviceConfig>,
    ) -> Self {
        match device_type {
            DeviceType::BulbTW => Device::Bulb(Bulb::TunableWhite(DeviceDefinition {
                features: if let Some(features) = features {
                    features
                } else {
                    DeviceFeatures {
                        hue: false,
                        color_temp: true,
                        effects: true,
                        dimming: true,
                        dual_head: false,
                    }
                },
                descriptor: if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    DeviceDescriptor {
                        module_name: None,
                        color_temp: None,
                        white_channels: None,
                        white_to_color_ratio: None,
                        firmware_version: None,
                        type_id_index: None,
                    }
                },
                config: if let Some(config) = config {
                    config
                } else {
                    DeviceConfig {
                        address: None,
                        mac: None,
                        name: None,
                        id: Some(Uuid::new_v4()),
                        state: DeviceState::Disabled,
                        speed: None,
                        scene: None,
                        total_intensity: Some(Intensity::from_percent(0)),
                        color_data: Some(ColorType::White(WhiteType::Tunable(WhiteTunableType {
                            temp: ColorTemp::new(
                                5000,
                                ColorTempSpace {
                                    min_temp: 1500,
                                    max_temp: 8000,
                                },
                            ),
                            intensity: Intensity::from_percent(0),
                        }))),
                    }
                },
            })),
            DeviceType::BulbDW => Device::Bulb(Bulb::DimmableWhite(DeviceDefinition {
                features: if let Some(features) = features {
                    features
                } else {
                    DeviceFeatures {
                        hue: false,
                        color_temp: false,
                        effects: false,
                        dimming: true,
                        dual_head: false,
                    }
                },
                descriptor: if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    DeviceDescriptor {
                        module_name: None,
                        color_temp: None,
                        white_channels: None,
                        white_to_color_ratio: None,
                        firmware_version: None,
                        type_id_index: None,
                    }
                },
                config: if let Some(config) = config {
                    config
                } else {
                    DeviceConfig {
                        address: None,
                        mac: None,
                        name: None,
                        id: Some(Uuid::new_v4()),
                        state: DeviceState::Disabled,
                        speed: None,
                        scene: None,
                        total_intensity: Some(Intensity::from_percent(0)),
                        color_data: Some(ColorType::White(WhiteType::Variable(
                            WhiteVariableType {
                                current: WhiteStaticType::Mixed,
                                ratio: 0.5,
                                intensity: Intensity::from_percent(0),
                            },
                        ))),
                    }
                },
            })),
            DeviceType::BulbRGB => Device::Bulb(Bulb::Rgb(DeviceDefinition {
                features: if let Some(features) = features {
                    features
                } else {
                    DeviceFeatures {
                        hue: true,
                        color_temp: true,
                        effects: true,
                        dimming: true,
                        dual_head: false,
                    }
                },
                descriptor: if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    DeviceDescriptor {
                        module_name: None,
                        color_temp: None,
                        white_channels: None,
                        white_to_color_ratio: None,
                        firmware_version: None,
                        type_id_index: None,
                    }
                },
                config: if let Some(config) = config {
                    config
                } else {
                    DeviceConfig {
                        address: None,
                        mac: None,
                        name: None,
                        id: Some(Uuid::new_v4()),
                        state: DeviceState::Disabled,
                        speed: None,
                        scene: None,
                        total_intensity: Some(Intensity::from_percent(0)),
                        color_data: Some(ColorType::Rgb(Rgb {
                            red: Intensity::from_percent(0),
                            green: Intensity::from_percent(0),
                            blue: Intensity::from_percent(0),
                        })),
                    }
                },
            })),
            DeviceType::Socket => Device::Socket(DeviceDefinition {
                features: if let Some(features) = features {
                    features
                } else {
                    DeviceFeatures {
                        hue: false,
                        color_temp: false,
                        effects: false,
                        dimming: false,
                        dual_head: false,
                    }
                },
                descriptor: if let Some(descriptor) = descriptor {
                    descriptor
                } else {
                    DeviceDescriptor {
                        module_name: None,
                        color_temp: None,
                        white_channels: None,
                        white_to_color_ratio: None,
                        firmware_version: None,
                        type_id_index: None,
                    }
                },
                config: if let Some(config) = config {
                    config
                } else {
                    DeviceConfig {
                        address: None,
                        mac: None,
                        name: None,
                        id: Some(Uuid::new_v4()),
                        state: DeviceState::Disabled,
                        speed: None,
                        scene: None,
                        total_intensity: None,
                        color_data: None,
                    }
                },
            }),
        }
    }

    pub fn from_descriptor(descriptor: DeviceDescriptor) -> Result<Self, DeviceError> {
        let mut device: Device = Device::new(DeviceType::BulbDW, None, None, None);
        let descriptor_bind = descriptor.clone();

        if let Some(name) = descriptor.module_name {
            let identifier = name
                .split("_")
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or(DeviceError::Parse(ParseError::UndefinedType(
                    descriptor_bind.clone(),
                )))?
                .clone();

            if identifier.contains(DEVICE_OPTS.rgb) {
                device = Device::new(DeviceType::BulbRGB, None, None, None);
            } else if identifier.contains(DEVICE_OPTS.tunable_white) {
                device = Device::new(DeviceType::BulbTW, None, None, None);
            } else if identifier.contains(DEVICE_OPTS.socket) {
                device = Device::new(DeviceType::Socket, None, None, None);
            } else {
                let effects = identifier.contains(DEVICE_OPTS.dual_head)
                    || identifier.contains(DEVICE_OPTS.single_head);
                let dual_head = identifier.contains(DEVICE_OPTS.dual_head);
                let patch = OptionalDeviceFeatures {
                    hue: None,
                    color_temp: None,
                    effects: Some(effects),
                    dimming: None,
                    dual_head: Some(dual_head),
                };
                device = device.patch_features(patch);
            }
        } else if let Some(type_id_index) = descriptor.type_id_index {
            let device_type = KNOWN_TYPE_IDS
                .get(type_id_index)
                .ok_or(DeviceError::Parse(ParseError::UndefinedType(
                    descriptor_bind.clone(),
                )))?
                .clone();
            device = Device::new(device_type, None, None, None);
            let patch = OptionalDeviceFeatures {
                hue: None,
                color_temp: None,
                effects: Some(true),
                dimming: None,
                dual_head: None,
            };
            device = device.patch_features(patch);
        }

        if let Some(color_temp) = descriptor_bind.color_temp {
            let descriptor = OptionalDeviceDescriptor {
                module_name: None,
                color_temp: Some(color_temp),
                firmware_version: None,
                white_channels: None,
                white_to_color_ratio: None,
                type_id_index: None,
            };

            device = device.patch_descriptor(descriptor);
        } else if device.get_type() == DeviceType::BulbRGB
            || device.get_type() == DeviceType::BulbTW
        {
            return Err(DeviceError::Parse(ParseError::ColorTemp(
                device.get_definition().descriptor,
            )));
        }

        return Ok(device);
    }

    pub fn patch_descriptor(self: Self, descriptor: OptionalDeviceDescriptor) -> Self {
        let mut definition = self.get_definition();
        let device_type = self.get_type();
        definition.descriptor.apply_options(descriptor);

        Device::new(
            device_type,
            Some(definition.features),
            Some(definition.descriptor),
            Some(definition.config),
        )
    }

    pub fn patch_features(self: Self, features: OptionalDeviceFeatures) -> Self {
        let mut definition = self.get_definition();
        let device_type = self.get_type();
        definition.features.apply_options(features);

        Device::new(
            device_type,
            Some(definition.features),
            Some(definition.descriptor),
            Some(definition.config),
        )
    }

    pub fn patch_config(self: Self, config: OptionalDeviceConfig) -> Self {
        let mut definition = self.get_definition();
        let device_type = self.get_type();
        definition.config.apply_options(config);

        Device::new(
            device_type,
            Some(definition.features),
            Some(definition.descriptor),
            Some(definition.config),
        )
    }

    pub fn get_definition(self: &Self) -> DeviceDefinition {
        match self {
            Self::Socket(definition) => definition.clone(),
            Self::Bulb(bulb) => match bulb {
                Bulb::DimmableWhite(definition) => definition.clone(),
                Bulb::TunableWhite(definition) => definition.clone(),
                Bulb::Rgb(definition) => definition.clone(),
            },
        }
    }

    pub fn get_type(self: &Self) -> DeviceType {
        match self {
            Self::Socket(_) => DeviceType::Socket,
            Self::Bulb(bulb) => match bulb {
                Bulb::TunableWhite(_) => DeviceType::BulbTW,
                Bulb::DimmableWhite(_) => DeviceType::BulbDW,
                Bulb::Rgb(_) => DeviceType::BulbRGB,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub enum GroupType {
    Group,
    Room,
    House,
}
#[derive(Debug, Clone)]
pub struct DeviceGroup {
    pub devices: Vec<Device>,
    pub name: String,
    pub id: Uuid,
    pub group_type: GroupType,
    pub network_config: NetworkConfig,
}

impl DeviceGroup {
    pub fn new(
        name: String,
        group_type: GroupType,
        devices: Option<Vec<Device>>,
        id: Option<Uuid>,
        network_config: Option<NetworkConfig>,
    ) -> Self {
        let devices_holder: Vec<Device>;
        let id_holder: Uuid;
        if let Some(devices) = devices {
            devices_holder = devices;
        } else {
            devices_holder = Vec::new();
        }

        if let Some(id) = id {
            id_holder = id;
        } else {
            id_holder = Uuid::new_v4();
        }

        return DeviceGroup {
            devices: devices_holder,
            name: name,
            id: id_holder,
            group_type: group_type,
            network_config: if let Some(network_config) = network_config {
                network_config
            } else {
                NetworkConfig {
                    timeout: DEFAULT_NETWORK_CONFIG.timeout,
                    max_sent_datagrams: DEFAULT_NETWORK_CONFIG.max_sent_datagrams,
                    first_send_interval: DEFAULT_NETWORK_CONFIG.first_send_interval,
                    max_backoff: DEFAULT_NETWORK_CONFIG.max_backoff,
                    keepalive: DEFAULT_NETWORK_CONFIG.keepalive,
                    port: DEFAULT_NETWORK_CONFIG.port,
                }
            },
        };
    }
}
