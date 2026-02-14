use serde::Deserialize;
use crate::errors::ServiceError;
use std::env;

#[derive(Debug, Deserialize, Clone)]
pub struct PrintConfig {
    pub id: String,
    pub device_type: String, // e.g., "esc_pos_network", "serial"
    pub connection: String,  // e.g., "192.168.1.100:9100" or "COM3"
}

#[derive(Debug, Deserialize, Clone)]
pub struct DrawerConfig {
    pub id: String,
    pub device_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DisplayConfig {
    pub id: String,
    pub device_type: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DevicesConfig {
    pub printers: Vec<PrintConfig>,
    pub drawers: Vec<DrawerConfig>,
    pub displays: Vec<DisplayConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub port: u16,
    pub auth_token: String,
    pub log_level: String,
    pub devices: DevicesConfig,
}

impl Settings {
    pub fn new() -> Result<Self, ServiceError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

        let s = config::Config::builder()
            .add_source(config::File::with_name("config"))
            // Add in settings from the environment (with a prefix of POS)
            // Eg.. `POS_DEBUG=1` would set the `debug` key
            .add_source(config::Environment::with_prefix("POS"))
            .build()?;

        s.try_deserialize().map_err(|e| ServiceError::ConfigError(e.to_string()))
    }
}
