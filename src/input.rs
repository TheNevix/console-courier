use std::io::{self, Write};
use crate::create_request::create_request_process;

pub fn listen_for_input() -> i8 {
    loop {
        print!("Enter your choice (1 for Create new request, 2 for Load requests): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                println!("You selected: Create new request");
                create_request_process();
            }
            "2" => {
                println!("You selected: Load requests");
                // Call function to load requests
                return 2;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
