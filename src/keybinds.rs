use console::Key;
use thiserror::Error;
use yaml_rust::Yaml;

#[derive(Error, Debug, Clone)]
pub enum KeybindError {
    #[error("Preset `{0}` does not exist.")]
    InvalidPreset(String),
}

pub struct Keybinds {
    pub up: Key,
    pub left: Key,
    pub down: Key,
    pub right: Key,
    pub cancel: Key,
    pub confirm: Key,
}

impl Keybinds {
    pub fn from_yaml(yaml: &Yaml) -> Result<Self, KeybindError> {
        let preset = (&yaml["keybinds"]["preset"]).clone().into_string();

        Ok(Keybinds::from_preset(match preset {
            Some(key) => KeybindPreset::from_str(key)?,
            None => KeybindPreset::Wasd,
        }))
    }

    pub fn from_preset(preset: KeybindPreset) -> Self {
        match preset {
            KeybindPreset::Arrows => Self {
                up: Key::ArrowUp,
                left: Key::ArrowLeft,
                down: Key::ArrowDown,
                right: Key::ArrowRight,
                cancel: Key::Escape,
                confirm: Key::Enter,
            },
            KeybindPreset::Hjkl => Self {
                up: Key::Char('h'),
                left: Key::Char('j'),
                down: Key::Char('k'),
                right: Key::Char('l'),
                cancel: Key::Escape,
                confirm: Key::Enter,
            },
            KeybindPreset::Wasd => Self {
                up: Key::Char('w'),
                left: Key::Char('a'),
                down: Key::Char('s'),
                right: Key::Char('d'),
                cancel: Key::Escape,
                confirm: Key::Enter,
            },
        }
    }
}

pub enum KeybindPreset {
    Arrows,
    Hjkl,
    Wasd,
}

impl KeybindPreset {
    pub fn from_str(key: String) -> Result<KeybindPreset, KeybindError> {
        match key.as_str() {
            "arrows" => Ok(KeybindPreset::Arrows),
            "hjkl" => Ok(KeybindPreset::Hjkl),
            "wasd" => Ok(KeybindPreset::Wasd),
            _ => Err(KeybindError::InvalidPreset(key)),
        }
    }
}
