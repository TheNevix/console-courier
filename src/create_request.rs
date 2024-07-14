use std::{fs, io};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Stdout, Write};
use std::path::{Path, PathBuf};
use chrono::Local;
use cliclr::{ConsoleLine, print_colored_text};
use cliclr::console_line::termcolor::{Color, ColorChoice, StandardStream};
use dialoguer::Select;
use rfd::FileDialog;
use crate::helpers::file_helpers::load_app_settings;
use crate::request::Request;
use crate::Settings;

pub fn create_request_process(){
    // Ask for method input
    clear_console();

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    print_colored_text(&ConsoleLine{ text: String::from("You are now creating a request, to go back enter q"), color: Color::Cyan }, &mut stdout);

    //request name
    print_colored_text(&ConsoleLine{ text: String::from("Enter a description for your request"), color: Color::Cyan }, &mut stdout);
    io::stdout().flush().unwrap();
    let mut desc = String::new();
    io::stdin().read_line(&mut desc).expect("Failed to read line");
    let desc = desc.trim().to_string();

    print_colored_text(&ConsoleLine{ text: String::from("Enter method: GET, POST, ..."), color: Color::Cyan }, &mut stdout);
    io::stdout().flush().unwrap();
    let mut method = String::new();
    io::stdin().read_line(&mut method).expect("Failed to read line");
    let method = method.trim().to_string();

    print_colored_text(&ConsoleLine{ text: String::from("Enter URL"), color: Color::Cyan }, &mut stdout);
    io::stdout().flush().unwrap();
    let mut url = String::new();
    io::stdin().read_line(&mut url).expect("Failed to read line");
    let url = url.trim().to_string();

    ///load settigns
    let settings = load_app_settings().expect("Failed to load settings");

    /// Ask user to select a JSON file
    let json_file_path: Option<PathBuf> = select_json_file(&settings);
    let json_file_name: Option<String> = json_file_path
        .as_ref()
        .and_then(|path| path.file_name())
        .and_then(|name| name.to_str())
        .map(String::from);

    let new_request = Request::new(desc.clone(), None, url.clone(), method.clone(), json_file_name);

    // Generate a unique filename for the request using the current time
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let request_file_name = format!("request_{}.json", timestamp);

    let request_file_path = Path::new(&settings.api_requests_path).join(request_file_name);

    save_request_to_file(&new_request, &request_file_path.to_str().expect("Failed to convert path to string"));

}
fn save_request_to_file(request: &Request, filename: &str) {
    let file_path = Path::new(filename);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .expect("Failed to create or open file");

    serde_json::to_writer_pretty(file, &request)
        .expect("Failed to write to file");
}

// Function to prompt user to select a JSON file using rfd
fn select_json_file(settings: &Settings) -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_directory(&settings.request_bodies_path)
        .pick_file()
}


pub fn clear_console(){
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

