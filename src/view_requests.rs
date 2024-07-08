use serde::{Deserialize, Serialize};
use std::fs::{self, read_dir};
use std::io::{self, Write};
use std::path::Path;
use crate::request::Request;
use crate::Settings;

pub fn view_requests_process() {
    let settings = load_settings("settings.json").expect("Failed to load settings");

    let request_dir = Path::new(&settings.api_requests_path);
    let entries = read_dir(request_dir).expect("Failed to read request directory");

    let mut requests = Vec::new();
    for entry in entries {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let data = fs::read_to_string(&path).expect("Failed to read request file");
            let request: Request = serde_json::from_str(&data).expect("Failed to deserialize request");
            requests.push(request);
        }
    }

    if requests.is_empty() {
        println!("No requests found.");
        return;
    }

    for (index, request) in requests.iter().enumerate() {
        println!("[{}] {}", index + 1, request.description);
    }

    println!("Enter the number of the request to see more details:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().parse::<usize>();

    match input {
        Ok(num) if num > 0 && num <= requests.len() => {
            println!("You selected request number {}: {}", num, requests[num - 1].description);
            // Here you can add further processing for the selected request
        }
        _ => println!("Invalid input."),
    }
}


fn load_settings(path: &str) -> io::Result<Settings> {
    let data = fs::read_to_string(path)?;
    let settings: Settings = serde_json::from_str(&data)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Failed to deserialize JSON: {}", e)))?;
    Ok(settings)
}
