use crate::{utils::input, Menus};

pub fn show_converters_menu() -> Menus {
    println!("--- CONVERTERS MENU ---");
    println!("[1] png to gif");
    println!("[2] png to pdf");

    // user input
    let input = input();
    let choice = input.trim();

    // auswahl
    match choice {
        "1" => {
            convert_png_to_gif();
            Menus::MACROS
        }
        "2" => {
            convert_png_to_pdf();
            Menus::MACROS
        }
        _ => Menus::MAIN,
    }
}

fn convert_png_to_gif() {
    println!("Converting PNG to GIF...");
}

fn convert_png_to_pdf() {
    println!("Converting PNG to PDF...");
}
