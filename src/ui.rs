use cliclr::console_line::termcolor::{Color, StandardStream};
use cliclr::{ConsoleLine, print_colored_text};

pub fn print_welcome_text(stdout: &mut StandardStream){
    print_colored_text(&ConsoleLine{ text: String::from("-----------------------------------------"), color: Color::Cyan }, stdout);
    print_colored_text(&ConsoleLine{ text: String::from("Welcome to ConsoleCourier. This application is a simplified postman replacement."), color: Color::Cyan }, stdout);
    print_colored_text(&ConsoleLine{ text: String::from("[1] Create new request     [2] Load requests"), color: Color::Cyan }, stdout);
    print_colored_text(&ConsoleLine{ text: String::from("-----------------------------------------"), color: Color::Cyan }, stdout);
}