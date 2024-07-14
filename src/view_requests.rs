use serde::{Deserialize, Serialize};
use std::fs::{self, read_dir};
use std::io::{self, Write};
use std::path::Path;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::{to_string_pretty, Value};
use crate::helpers::file_helpers::load_app_settings;
use crate::request::Request;
use crate::Settings;

pub fn view_requests_process() {
    let settings = load_app_settings().expect("Failed to load settings");

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
            let selected_request = &requests[num - 1];
            println!("You selected request number {}: {}", num, selected_request.description);
            println!("URL: {}", selected_request.url);
            println!("Method: {}", selected_request.method);
            println!("Body file: {}", selected_request.body_file_name.as_deref().unwrap_or("None"));

            println!("Do you want to send this request? (yes/no):");
            let mut send_input = String::new();
            io::stdin().read_line(&mut send_input).expect("Failed to read line");
            if send_input.trim().eq_ignore_ascii_case("yes") {
                send_request(selected_request);
            }
        }
        _ => println!("Invalid input."),
    }
}


fn send_request(request: &Request) {
    let client = reqwest::blocking::Client::new();

    let mut headers = HeaderMap::new();
    if let Some(hdrs) = &request.headers {
        for (key, value) in hdrs {
            headers.insert(key.parse::<HeaderName>().unwrap(), value.parse::<HeaderValue>().unwrap());
        }
    }

    let body = if let Some(body_file_path) = &request.body_file_name {
        fs::read_to_string(body_file_path).expect("Failed to read body file")
    } else {
        String::new()
    };

    let response = match request.method.as_str() {
        "GET" => client.get(&request.url).headers(headers).send(),
        "POST" => client.post(&request.url).headers(headers).body(body).send(),
        "PUT" => client.put(&request.url).headers(headers).body(body).send(),
        "DELETE" => client.delete(&request.url).headers(headers).send(),
        _ => {
            println!("Unsupported HTTP method: {}", request.method);
            return;
        }
    };

    match response {
        Ok(resp) => {
            println!("Response Status: {}", resp.status());
            let resp_text = resp.text().unwrap_or_else(|_| "Failed to read response body".to_string());

            // Attempt to pretty-print the JSON response
            match serde_json::from_str::<Value>(&resp_text) {
                Ok(json) => {
                    let pretty_json = to_string_pretty(&json).unwrap_or_else(|_| "Failed to format JSON".to_string());
                    println!("Response Body: {}", pretty_json);
                }
                Err(_) => {
                    println!("Response Body: {}", resp_text);
                }
            }
        }
        Err(err) => {
            println!("Request failed: {}", err);
        }
    }
}