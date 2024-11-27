use std::io::{self, Write};

pub fn update_terminal() {
    print!("\x1B[H");
}

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[H");
    io::stdout().flush().unwrap();
}

pub fn wait_input() {
    println!("Press \x1b[32mENTER\x1b[0m:");
    let mut next_str: String = String::new();
    io::stdin()
        .read_line(&mut next_str)
        .expect("Sorry, can't read line.");
}
