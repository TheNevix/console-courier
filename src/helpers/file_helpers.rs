use std::{fs, io};
use crate::constants::APP_SETTINGS_FILE_NAME;
use crate::Settings;


pub fn load_app_settings() -> io::Result<Settings> {
    let data = fs::read_to_string(APP_SETTINGS_FILE_NAME)?;
    let settings: Settings = serde_json::from_str(&data)?;
    Ok(settings)
}

pub fn create_app_settings(settings: &Settings) -> io::Result<()> {
    let data = serde_json::to_string_pretty(settings)?;
    fs::write(APP_SETTINGS_FILE_NAME, data)?;
    Ok(())
}