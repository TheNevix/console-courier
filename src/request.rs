use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request{
    pub description: String,
    pub headers: Option<HashMap<String, String>>,
    pub url: String,
    pub method: String,
    pub body_file_name: Option<String>
}

impl Request {
    pub fn new(description: String, headers: Option<HashMap<String, String>>, url: String, method: String, body_file_name: Option<String>) -> Self {
        Request { description, headers, url, method, body_file_name }
    }
}