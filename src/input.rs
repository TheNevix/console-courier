use std::io::{self, Write};

pub fn listen_for_input() {
    loop {
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "1" => {
                println!("You selected: Create new request");
                // Call function to create a new request
            }
            "2" => {
                println!("You selected: Load requests");
                // Call function to load requests
            }
            "q" | "quit" | "exit" => {
                println!("Exiting the application.");
                break;
            }
            _ => {
                println!("Invalid choice, please try again.");
            }
        }
    }
}
