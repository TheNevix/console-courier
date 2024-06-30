mod ui;
mod input;
mod create_request;
mod request;

use cliclr::console_line::termcolor::{ColorChoice, StandardStream};
use ui::print_welcome_text;
use crate::input::listen_for_input;

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    print_welcome_text(&mut stdout);

    let myChoice = listen_for_input();
    println!("{}", myChoice);
}