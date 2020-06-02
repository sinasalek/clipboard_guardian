extern crate confy;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfyConfig {
    pub interval: u64,
    pub history_total: u64,
    pub restore_hot_key: String,
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            interval: 15,
            history_total: 5,
            restore_hot_key: String::from("ctrl+shift+J")
        }
    }
}

pub fn config_get() -> Result<ConfyConfig, confy::ConfyError> {
    let cfg: ConfyConfig = confy::load("clipboard_guardian")?;
    Ok(cfg)
}