use std::{fs, io};

use thiserror::Error;
use yaml_rust::YamlLoader;

use crate::{
    keybinds::{KeybindError, Keybinds},
    strategems::{Strategem, StrategemError},
};

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Could not find config file at `{0}`.")]
    ConfigNotFound(#[from] io::Error),

    #[error("`{0}` is not a valid YAML file.")]
    InvalidYaml(#[from] yaml_rust::ScanError),

    #[error("Strategem configuration is invalid: {0}")]
    InvalidStrategems(#[from] StrategemError),

    #[error("Keybind configuration is invalid: {0}")]
    InvalidKeybinds(#[from] KeybindError),
}

pub struct Config {
    pub keybinds: Keybinds,
    pub strategems: Vec<Strategem>,
}

impl Config {
    pub fn load_from_yaml(path: &str) -> Result<Self, ConfigError> {
        let file = fs::read_to_string(path)?;
        let yaml = YamlLoader::load_from_str(&file)?;

        let keybinds = Keybinds::from_yaml(&yaml[0])?;
        let strategems = Strategem::from_yaml(&yaml[0])?;

        Ok(Self {
            keybinds,
            strategems,
        })
    }
}
