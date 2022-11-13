use super::wiz_errors::{
    DeviceColorTempParseError, DeviceTypeParseError, SceneIDError, SceneNameError,
};
use anyhow::Error;
use bytemuck::{cast, try_cast};
use http::Uri;
use lazy_static::lazy_static;
use macaddr::MacAddr6;
use num::FromPrimitive;
use num_derive::{FromPrimitive, ToPrimitive};
use optional_struct::OptionalStruct;
use std::collections::HashMap;
use uuid::Uuid;

lazy_static! {
    static ref SCENES: HashMap<&'static str, u32> = HashMap::from([
        ("Ocean", 1),
        ("Romance", 2),
        ("Sunset", 3),
        ("Party", 4),
        ("Fireplace", 5),
        ("Cozy", 6),
        ("Forest", 7),
        ("Pastel Colors", 8),
        ("Wake up", 9),
        ("Bedtime", 10),
        ("Warm White", 11),
        ("Daylight", 12),
        ("Cool white", 13),
        ("Night light", 14),
        ("Focus", 15),
        ("Relax", 16),
        ("True colors", 17),
        ("TV time", 18),
        ("Plantgrowth", 19),
        ("Spring", 20),
        ("Summer", 21),
        ("Fall", 22),
        ("Deepdive", 23),
        ("Jungle", 24),
        ("Mojito", 25),
        ("Club", 26),
        ("Christmas", 27),
        ("Halloween", 28),
        ("Candlelight", 29),
        ("Golden white", 30),
        ("Pulse", 31),
        ("Steampunk", 32),
        ("Rhythm", 1000),
    ]);
    static ref DEVICE_OPTS: DeviceOptions = DeviceOptions {
        rgb: "RGB",
        dimmable_white: "DW",
        tunable_white: "TW",
        dual_head: "DH",
        single_head: "SH",
        socket: "SOCKET"
    };
    static ref KNOWN_TYPE_IDS: Vec<DeviceType> = vec![DeviceType::BulbDW];
    static ref WIZMOTE_BUTTON_MAP: HashMap<&'static str, &'static str> = HashMap::from([
        ("wfa1", "on"),
        ("wfa2", "off"),
        ("wfa3", "night"),
        ("wfa8", "decrease_brightness"),
        ("wfa9", "increase_brightness"),
        ("wfa16", "1"),
        ("wfa17", "2"),
        ("wfa18", "3"),
        ("wfa19", "4"),
    ]);
}

#[derive(Debug, Copy, Clone)]
pub struct DeviceOptions {
    pub rgb: &'static str,
    pub dimmable_white: &'static str,
    pub tunable_white: &'static str,
    pub dual_head: &'static str,
    pub single_head: &'static str,
    pub socket: &'static str,
}

#[derive(FromPrimitive, ToPrimitive, Debug, Clone, Copy)]
pub enum Scenes {
    Ocean = 1,
    Romance = 2,
    Sunset = 3,
    Party = 4,
    Fireplace = 5,
    Cozy = 6,
    Forest = 7,
    PastelColors = 8,
    Wakeup = 9,
    Bedtime = 10,
    WarmWhite = 11,
    Daylight = 12,
    CoolWhite = 13,
    NighLight = 14,
    Focus = 15,
    Relax = 16,
    Truecolors = 17,
    TVtime = 18,
    Plantgrowth = 19,
    Spring = 20,
    Summer = 21,
    Fall = 22,
    Deepdive = 23,
    Jungle = 24,
    Mojito = 25,
    Club = 26,
    Christmas = 27,
    Halloween = 28,
    Candlelight = 29,
    GoldenWhite = 30,
    Pulse = 31,
    Steampunk = 32,
    Rhythm = 1000,
}

impl Scenes {
    pub fn from_id(id: u32) -> Result<Self, Error> {
        let scene: Option<Self> = FromPrimitive::from_u32(id);
        return scene.ok_or(Error::new(SceneIDError {
            given_id: id,
            details: "Scene ID out of range. Expected 1-32 or 1000".into(),
        }));
    }

    pub fn from_name(name: String) -> Result<Self, Error> {
        let scene_name = name.clone();
        let id = SCENES
            .get::<&str>(&name.as_str())
            .ok_or(Error::new(SceneNameError {
                given_name: scene_name.into(),
                details: "Scene name not found.".into(),
            }))?;
        return Scenes::from_id(*id);
    }

    fn get_scenes_list(list: Vec<u32>) -> Result<Vec<Self>, Error> {
        let mut scenes: Vec<Scenes> = Vec::new();

        for scene_id in list {
            let scene = &Scenes::from_id(scene_id)?;
            scenes.push(*scene);
        }
        return Ok(scenes);
    }

    pub fn get_tunable_white_scenes() -> Result<Vec<Self>, Error> {
        let tunable_white_scenes = vec![6, 9, 10, 11, 12, 13, 14, 15, 16, 18, 29, 30, 31, 32];
        return Scenes::get_scenes_list(tunable_white_scenes);
    }

    pub fn get_dimmable_white_scenes() -> Result<Vec<Self>, Error> {
        let dimmable_white_scenes = vec![9, 10, 13, 14, 29, 30, 31, 32];
        return Scenes::get_scenes_list(dimmable_white_scenes);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ColorTempSpace {
    pub min_temp: u16,
    pub max_temp: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorTemp {
    pub value: u16,
    pub space: ColorTempSpace,
}

impl ColorTemp {
    pub fn new(value: u16, space: ColorTempSpace) -> Self {
        ColorTemp {
            value: value.clamp(space.min_temp, space.max_temp),
            space: space,
        }
    }
    pub fn delta(mut self: Self, delta: i32) {
        self.value = cast::<i32, u16>(
            (cast::<u16, i32>(self.value) + delta)
                .clamp(self.space.min_temp.into(), self.space.max_temp.into()),
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Intensity {
    pub raw_value: u8,
    pub percentage: u8,
}

impl Intensity {
    pub fn from_raw(value: u8) -> Self {
        Intensity {
            raw_value: value,
            percentage: Intensity::u8_to_p100(value),
        }
    }
    pub fn from_percent(value: u8) -> Self {
        Intensity {
            percentage: value,
            raw_value: Intensity::p100_to_u8(value),
        }
    }
    pub fn delta_raw(mut self: Self, delta: i16) {
        self.raw_value = cast::<i16, u8>((cast::<u8, i16>(self.raw_value) + delta).clamp(0, 255));
        self.update_p100();
    }
    pub fn delta_percent(mut self: Self, delta: i16) {
        self.percentage = cast::<i16, u8>((cast::<u8, i16>(self.percentage) + delta).clamp(0, 100));
        self.update_u8();
    }
    fn update_u8(mut self: Self) {
        self.raw_value = Intensity::p100_to_u8(self.percentage);
    }
    fn update_p100(mut self: Self) {
        self.percentage = Intensity::u8_to_p100(self.raw_value);
    }
    fn p100_to_u8(percent: u8) -> u8 {
        return cast::<f64, u8>((cast::<u8, f64>(percent.clamp(0, 100)) * 2.55).floor());
    }
    fn u8_to_p100(raw: u8) -> u8 {
        return cast::<f64, u8>((cast::<u8, f64>(raw.clamp(0, 255)) / 2.55).floor());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WhiteStaticType {
    Warm,
    Cold,
    Mixed,
}

#[derive(Debug, Clone, Copy)]
pub struct WhiteVariableType {
    pub current: WhiteStaticType,
    pub ratio: f32,
    pub intensity: Intensity,
}

impl WhiteVariableType {
    pub fn new() -> Self {
        Self {
            current: WhiteStaticType::Mixed,
            ratio: 0.0,
            intensity: Intensity::from_percent(0),
        }
    }
    pub fn delta_percent(mut self: Self, delta: i16) {
        self.intensity.delta_percent(delta);
    }
    pub fn delta_raw(mut self: Self, delta: i16) {
        self.intensity.delta_raw(delta);
    }

    pub fn patch_percent(mut self: Self, value: u8) {
        self.intensity = Intensity::from_percent(value);
    }

    pub fn patch_raw(mut self: Self, value: u8) {
        self.intensity = Intensity::from_raw(value);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct WhiteTunableType {
    pub temp: ColorTemp,
    pub intensity: Intensity,
}

impl WhiteTunableType {
    pub fn new(space: ColorTempSpace) -> Self {
        Self {
            temp: ColorTemp::new(5000, space),
            intensity: Intensity::from_percent(0),
        }
    }
    pub fn delta_percent(mut self: Self, delta: i16) {
        self.intensity.delta_percent(delta);
    }
    pub fn delta_raw(mut self: Self, delta: i16) {
        self.intensity.delta_raw(delta);
    }

    pub fn patch_percent(mut self: Self, value: u8) {
        self.intensity = Intensity::from_percent(value);
    }

    pub fn patch_raw(mut self: Self, value: u8) {
        self.intensity = Intensity::from_raw(value);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WhiteType {
    Tunable(WhiteTunableType),
    Dimmable(WhiteStaticType),
    Variable(WhiteVariableType),
}

#[derive(Debug, Clone, Copy)]
pub enum ColorChannel {
    Red,
    Green,
    Blue,
    WhiteCold,
    WhiteHot,
}

#[derive(Debug, Clone, Copy, OptionalStruct)]
#[optional_derive(Debug, Clone, Copy)]
pub struct Rgb {
    pub red: Intensity,
    pub green: Intensity,
    pub blue: Intensity,
}

impl Rgb {
    pub fn new_from_percent(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: Intensity::from_percent(r),
            green: Intensity::from_percent(g),
            blue: Intensity::from_percent(b),
        }
    }
    pub fn new_from_raw(r: u8, g: u8, b: u8) -> Self {
        Self {
            red: Intensity::from_raw(r),
            green: Intensity::from_raw(g),
            blue: Intensity::from_raw(b),
        }
    }
    pub fn patch_delta_percent(mut self: Self, channel: ColorChannel, delta: i16) {
        match channel {
            ColorChannel::Red => self.red.delta_percent(delta),
            ColorChannel::Green => self.green.delta_percent(delta),
            ColorChannel::Blue => self.blue.delta_percent(delta),
            _ => {}
        }
    }
    pub fn patch_delta_raw(mut self: Self, channel: ColorChannel, delta: i16) {
        match channel {
            ColorChannel::Red => self.red.delta_raw(delta),
            ColorChannel::Green => self.green.delta_raw(delta),
            ColorChannel::Blue => self.blue.delta_raw(delta),
            _ => {}
        }
    }
    pub fn patch_value_percent(mut self: Self, channel: ColorChannel, value: u8) {
        match channel {
            ColorChannel::Red => self.red = Intensity::from_percent(value),
            ColorChannel::Green => self.green = Intensity::from_percent(value),
            ColorChannel::Blue => self.blue = Intensity::from_percent(value),
            _ => {}
        }
    }
    pub fn patch_value_raw(mut self: Self, channel: ColorChannel, value: u8) {
        match channel {
            ColorChannel::Red => self.red = Intensity::from_raw(value),
            ColorChannel::Green => self.green = Intensity::from_raw(value),
            ColorChannel::Blue => self.blue = Intensity::from_raw(value),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgbw {
    pub red: Intensity,
    pub green: Intensity,
    pub blue: Intensity,
    pub white: WhiteVariableType,
}

impl Rgbw {
    pub fn new_from_percent(r: u8, g: u8, b: u8, w: u8) -> Self {
        Self {
            red: Intensity::from_percent(r),
            green: Intensity::from_percent(g),
            blue: Intensity::from_percent(b),
            white: WhiteVariableType {
                current: WhiteStaticType::Mixed,
                ratio: 0.0,
                intensity: Intensity::from_percent(w),
            },
        }
    }
    pub fn new_from_raw(r: u8, g: u8, b: u8, w: u8) -> Self {
        Self {
            red: Intensity::from_raw(r),
            green: Intensity::from_raw(g),
            blue: Intensity::from_raw(b),
            white: WhiteVariableType {
                current: WhiteStaticType::Mixed,
                ratio: 0.0,
                intensity: Intensity::from_raw(w),
            },
        }
    }
    pub fn patch_delta_percent(mut self: Self, channel: ColorChannel, delta: i16) {
        match channel {
            ColorChannel::Red => self.red.delta_percent(delta),
            ColorChannel::Green => self.green.delta_percent(delta),
            ColorChannel::Blue => self.blue.delta_percent(delta),
            ColorChannel::WhiteCold => self.white.delta_percent(delta),
            _ => {}
        }
    }
    pub fn patch_delta_raw(mut self: Self, channel: ColorChannel, delta: i16) {
        match channel {
            ColorChannel::Red => self.red.delta_raw(delta),
            ColorChannel::Green => self.green.delta_raw(delta),
            ColorChannel::Blue => self.blue.delta_raw(delta),
            ColorChannel::WhiteCold => self.white.delta_raw(delta),
            _ => {}
        }
    }
    pub fn patch_value_percent(mut self: Self, channel: ColorChannel, value: u8) {
        match channel {
            ColorChannel::Red => self.red = Intensity::from_percent(value),
            ColorChannel::Green => self.green = Intensity::from_percent(value),
            ColorChannel::Blue => self.blue = Intensity::from_percent(value),
            ColorChannel::WhiteCold => self.white.patch_percent(value),
            _ => {}
        }
    }
    pub fn patch_value_raw(mut self: Self, channel: ColorChannel, value: u8) {
        match channel {
            ColorChannel::Red => self.red = Intensity::from_raw(value),
            ColorChannel::Green => self.green = Intensity::from_raw(value),
            ColorChannel::Blue => self.blue = Intensity::from_raw(value),
            ColorChannel::WhiteCold => self.white.patch_raw(value),
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rgbww {
    pub red: Intensity,
    pub green: Intensity,
    pub blue: Intensity,
    pub white_cold: WhiteVariableType,
    pub white_hot: WhiteVariableType,
}

impl Rgbww {
    pub fn new_from_percent(r: u8, g: u8, b: u8, wc: u8, wh: u8) -> Self {
        Self {
            red: Intensity::from_percent(r),
            green: Intensity::from_percent(g),
            blue: Intensity::from_percent(b),
            white_cold: WhiteVariableType {
                current: WhiteStaticType::Mixed,
                ratio: 0.0,
                intensity: Intensity::from_percent(wc),
            },
            white_hot: WhiteVariableType {
                current: WhiteStaticType::Mixed,
                ratio: 0.0,
                intensity: Intensity::from_percent(wh),
            },
        }
    }
    pub fn new_from_raw(r: u8, g: u8, b: u8, wc: u8, wh: u8) -> Self {
        Self {
            red: Intensity::from_raw(r),
            green: Intensity::from_raw(g),
            blue: Intensity::from_raw(b),
            white_cold: WhiteVariableType {
                current: WhiteStaticType::Mixed,
                ratio: 0.0,
                intensity: Intensity::from_raw(wc),
            },
            white_hot: WhiteVariableType {
                current: WhiteStaticType::Mixed,
                ratio: 0.0,
                intensity: Intensity::from_percent(wh),
            },
        }
    }
    pub fn patch_delta_percent(mut self: Self, channel: ColorChannel, delta: i16) {
        match channel {
            ColorChannel::Red => self.red.delta_percent(delta),
            ColorChannel::Green => self.green.delta_percent(delta),
            ColorChannel::Blue => self.blue.delta_percent(delta),
            ColorChannel::WhiteCold => self.white_cold.delta_percent(delta),
            ColorChannel::WhiteHot => self.white_hot.delta_percent(delta),
        }
    }
    pub fn patch_delta_raw(mut self: Self, channel: ColorChannel, delta: i16) {
        match channel {
            ColorChannel::Red => self.red.delta_raw(delta),
            ColorChannel::Green => self.green.delta_raw(delta),
            ColorChannel::Blue => self.blue.delta_raw(delta),
            ColorChannel::WhiteCold => self.white_cold.delta_raw(delta),
            ColorChannel::WhiteHot => self.white_hot.delta_raw(delta),
        }
    }
    pub fn patch_value_percent(mut self: Self, channel: ColorChannel, value: u8) {
        match channel {
            ColorChannel::Red => self.red = Intensity::from_percent(value),
            ColorChannel::Green => self.green = Intensity::from_percent(value),
            ColorChannel::Blue => self.blue = Intensity::from_percent(value),
            ColorChannel::WhiteCold => self.white_cold.patch_percent(value),
            ColorChannel::WhiteHot => self.white_hot.patch_percent(value),
        }
    }
    pub fn patch_value_raw(mut self: Self, channel: ColorChannel, value: u8) {
        match channel {
            ColorChannel::Red => self.red = Intensity::from_raw(value),
            ColorChannel::Green => self.green = Intensity::from_raw(value),
            ColorChannel::Blue => self.blue = Intensity::from_raw(value),
            ColorChannel::WhiteCold => self.white_cold.patch_raw(value),
            ColorChannel::WhiteHot => self.white_hot.patch_raw(value),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColorType {
    Rgb(Rgb),
    Rgbw(Rgbw),
    Rgbww(Rgbww),
    White(WhiteType),
}

#[derive(Debug, Clone, Copy)]
pub enum DeviceState {
    Enabled,
    Disabled,
    Unknown,
}

#[derive(Debug, Clone, Copy, OptionalStruct)]
#[optional_derive(Debug, Clone, Copy)]
pub struct DeviceFeatures {
    pub hue: bool,
    pub color_temp: bool,
    pub effects: bool,
    pub dimming: bool,
    pub dual_head: bool,
}

#[derive(Debug, Clone, OptionalStruct)]
#[optional_derive(Debug, Clone)]
pub struct DeviceDescriptor {
    pub module_name: Option<String>,
    pub color_temp: Option<ColorTempSpace>,
    pub firmware_version: Option<String>,
    pub white_channels: Option<u16>,
    pub white_to_color_ratio: Option<u16>,
    pub type_id_index: Option<usize>,
}

#[derive(Debug, Clone, OptionalStruct)]
#[optional_derive(Debug, Clone)]
pub struct DeviceConfig {
    pub address: Option<Uri>,
    pub mac: Option<MacAddr6>,
    pub name: Option<String>,
    pub id: Option<Uuid>,
    pub state: DeviceState,
    pub speed: Option<u32>,
    pub scene: Option<Scenes>,
    pub total_intensity: Option<Intensity>,
    pub color_data: Option<ColorType>,
}

#[derive(Debug, Clone)]
pub struct DeviceDefinition {
    pub features: DeviceFeatures,
    pub descriptor: DeviceDescriptor,
    pub config: DeviceConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    BulbTW,
    BulbDW,
    BulbRGB,
    Socket,
}

#[derive(Debug, Clone)]
pub enum Device {
    Bulb(Bulb),
    Socket(DeviceDefinition),
}

#[derive(Debug, Clone)]
pub enum Bulb {
    TunableWhite(DeviceDefinition),
    DimmableWhite(DeviceDefinition),
    Rgb(DeviceDefinition),
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

    pub fn from_descriptor(descriptor: DeviceDescriptor) -> Result<Self, Error> {
        let mut device: Device = Device::new(DeviceType::BulbDW, None, None, None);
        let descriptor_bind = descriptor.clone();

        if let Some(name) = descriptor.module_name {
            let identifier = name
                .split("_")
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or(Error::new(DeviceTypeParseError {
                    data: descriptor_bind.clone(),
                    details: "Failed to find an identifier in the descriptor.".to_string(),
                }))?
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
                .ok_or(Error::new(DeviceTypeParseError {
                    data: descriptor,
                    details: "Failed finding a known type ID in the descriptor".to_string(),
                }))?
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
            return Err(Error::new(DeviceColorTempParseError {
                data: device.get_definition().descriptor,
                details: "Bulb type should include color temp data in the descriptor.".to_string(),
            }));
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
}

impl DeviceGroup {
    pub fn new(
        name: String,
        group_type: GroupType,
        devices: Option<Vec<Device>>,
        id: Option<Uuid>,
    ) -> Self {
        let mut devices_holder: Vec<Device>;
        let mut id_holder: Uuid;
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
        };
    }
}
