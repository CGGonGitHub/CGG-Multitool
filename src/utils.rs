use std::io::{self, Write};

pub fn input() -> String {
    let mut choice = String::new();
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");
    // clear screen
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    choice
}
