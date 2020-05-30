extern crate confy;
use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfyConfig {
    pub interval: u64
}

impl Default for ConfyConfig {
    fn default() -> Self {
        ConfyConfig {
            interval: 15
        }
    }
}

pub fn config_get() -> Result<ConfyConfig, confy::ConfyError> {
    let cfg: ConfyConfig = confy::load("clipboard_vanisher")?;
    Ok(cfg)
}