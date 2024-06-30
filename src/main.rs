mod ui;
mod input;

use cliclr::console_line::termcolor::{ColorChoice, StandardStream};
use ui::print_welcome_text;
use crate::input::listen_for_input;

fn main() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    print_welcome_text(&mut stdout);

    listen_for_input();
}