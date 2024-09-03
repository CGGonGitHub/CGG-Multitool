// modules
use std::io;
// files
mod macros;
mod test;

const OPTIONS: [&str; 3] = ["exit", "test", "macro"];
const FILES: [fn(); 3] = [exit_program, test::test_main, macros::macros_main];

pub fn clear_terminal() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn exit_program() {
    println!("I will add this later");
}

pub fn menu() {
    clear_terminal();
    println!("Welcome to CoolGermanGuy's Multitool");
    for (index, option) in OPTIONS.iter().enumerate() {
        println!("[{}] {option}", index + 1);
    }
    print!("\n");

    // get input
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();
    
    // check if option exists and run the function
    for (index, option) in OPTIONS.iter().enumerate() {

        if &input == option || input == (index + 1).to_string() {
            FILES[index]();
        }
    }

}

fn main() {
    menu();
}
