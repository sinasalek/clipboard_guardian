extern crate confy;
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
struct AppConfig {
    interval: u64
}

/// `AppConfig` implements `Default`
impl ::std::default::Default for AppConfig {
    fn default() -> Self { AppConfig {
        interval: 15
    } }
}

pub fn config_get() -> Result<(), confy::ConfyError> {
    let cfg = confy::load("clipboard_vanisher");
    cfg
}