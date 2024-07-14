mod ui;
mod input;
mod create_request;
mod request;
mod view_requests;
mod helpers;
mod constants;

use helpers::file_helpers::{load_app_settings, create_app_settings};

use std::{fs, io};
use std::path::Path;
use cliclr::console_line::termcolor::{ColorChoice, StandardStream};
use rfd::FileDialog;
use serde::{Deserialize, Serialize};
use ui::print_welcome_text;
use crate::constants::APP_SETTINGS_FILE_NAME;
use crate::input::listen_for_input;

#[derive(Serialize, Deserialize)]
struct Settings {
    api_requests_path: String,
    request_bodies_path: String,
}

fn main() {
    let settings = if Path::new(APP_SETTINGS_FILE_NAME).exists() {
        load_app_settings().expect("Failed to load settings")
    } else {
        let api_requests_path = prompt_for_directory("Select a directory to save API requests as JSON");
        let request_bodies_path = prompt_for_directory("Select a directory to look for JSON request bodies");
        let settings = Settings {
            api_requests_path,
            request_bodies_path,
        };
        create_app_settings(&settings).expect("Failed to save settings");
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