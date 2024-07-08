mod ui;
mod input;
mod create_request;
mod request;
mod view_requests;

use std::{fs, io};
use std::path::Path;
use cliclr::console_line::termcolor::{ColorChoice, StandardStream};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use ui::print_welcome_text;
use crate::input::listen_for_input;

#[derive(Serialize, Deserialize)]
struct Settings {
    api_requests_path: String,
    request_bodies_path: String,
}

fn main() {
    let settings_path = "settings.json";

    let settings = if Path::new(settings_path).exists() {
        load_settings(settings_path).expect("Failed to load settings")
    } else {
        let api_requests_path = prompt_for_directory("Select a directory to save API requests as JSON");
        let request_bodies_path = prompt_for_directory("Select a directory to look for JSON request bodies");
        let settings = Settings {
            api_requests_path,
            request_bodies_path,
        };
        save_settings(settings_path, &settings).expect("Failed to save settings");
        settings
    };

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    print_welcome_text(&mut stdout);

    let myChoice = listen_for_input();
    println!("{}", myChoice);
}

fn prompt_for_directory(prompt: &str) -> String {
    println!("{}", prompt);
    let path = FileDialog::new()
        .set_title(prompt)
        .pick_folder()
        .expect("Failed to select a directory");

    path.to_str().expect("Failed to convert path to string").to_string()
}

fn load_settings(path: &str) -> io::Result<Settings> {
    let data = fs::read_to_string(path)?;
    let settings: Settings = serde_json::from_str(&data)?;
    Ok(settings)
}

fn save_settings(path: &str, settings: &Settings) -> io::Result<()> {
    let data = serde_json::to_string_pretty(settings)?;
    fs::write(path, data)?;
    Ok(())
}