// modules definieren
mod submenus;
mod utils;

// funktionen aus submenu module importieren
use submenus::{show_converters_menu, show_macro_menu};
use utils::input;

// menus
enum Menus {
    MAIN,
    MACROS,
    CONVERTERS,
    EXIT,
}

// macro toggles
struct MacroToggles {
    auto_clicker: bool,
    spam_e: bool,
}

fn main() {
    // zurzeitiges menu
    let mut current_menu = Menus::MAIN;

    // macro toggles state
    let mut macro_toggles = MacroToggles {
        auto_clicker: false,
        spam_e: false,
    };

    loop {
        // anzeige vom jeztigen menu
        let next_menu = match current_menu {
            Menus::MAIN => show_main_menu(), // main menu aus main.rs file
            Menus::MACROS => show_macro_menu(&mut macro_toggles), // macro menu aus submenu/macros.rs
            Menus::CONVERTERS => show_converters_menu(), // converters menu aus submenu/converters.rs
            Menus::EXIT => {
                println!("Goodbye!");
                std::process::exit(0);
            }
        };

        // menu wechsel
        current_menu = next_menu;
    }
}

fn show_main_menu() -> Menus {
    println!("--- MAIN MENU ---");
    println!("[1] macros");
    println!("[2] converters");
    println!("[e] Exit");

    // user input
    let input = input();
    let choice = input.trim();

    // auswahl
    match choice {
        "1" => Menus::MACROS,
        "2" => Menus::CONVERTERS,
        "e" => {
            println!("Goodbye!");
            std::process::exit(0);
        }
        _ => Menus::MAIN,
    }
}
