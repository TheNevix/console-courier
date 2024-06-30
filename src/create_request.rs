use std::{fs, io};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Stdout, Write};
use std::path::{Path, PathBuf};
use cliclr::{ConsoleLine, print_colored_text};
use cliclr::console_line::termcolor::{Color, ColorChoice, StandardStream};
use dialoguer::Select;
use rfd::FileDialog;
use crate::request::Request;

pub fn create_request_process(){
    // Ask for method input
    clear_console();

    let mut request_map = load_existing_requests("settings.json");

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

    // Ask user to select a JSON file
    let json_file_path = select_json_file().expect("No file selected or invalid file");

    // Read the contents of the selected JSON file as a string
    let body = fs::read_to_string(&json_file_path)
        .expect("Failed to read file")
        .trim()
        .to_string();

    let new_request = Request::new(desc.clone(), None, url.clone(), method.clone(), body.clone(), json_file_path.to_string_lossy().to_string());

    // Generate a unique key for the request (you can use a UUID library for a more robust approach)
    let key = format!("request_{}", request_map.len() + 1);

    // Insert the Request into the HashMap
    request_map.insert(key.clone(), new_request);

    // Save the request map to settings.json
    save_requests_to_file(&request_map, "settings.json");

}

// Function to save the request map to a JSON file
fn save_requests_to_file(requests: &HashMap<String, Request>, filename: &str) {
    let file_path = Path::new(filename);

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .expect("Failed to create or open file");

    serde_json::to_writer_pretty(file, &requests)
        .expect("Failed to write to file");
}

// Function to load existing requests from a JSON file if it exists
fn load_existing_requests(filename: &str) -> HashMap<String, Request> {
    let file_path = Path::new(filename);

    if file_path.exists() {
        let mut file = File::open(file_path).expect("Failed to open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file");
        serde_json::from_str(&content).expect("Failed to deserialize JSON")
    } else {
        HashMap::new()
    }
}


// Function to prompt user to select a JSON file using rfd
fn select_json_file() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("JSON", &["json"])
        .set_directory(".")
        .pick_file()
}


pub fn clear_console(){
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}