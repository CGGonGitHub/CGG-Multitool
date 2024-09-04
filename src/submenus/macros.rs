use crate::{utils::input, MacroToggles, Menus};

pub fn show_macro_menu(toggle_states: &mut MacroToggles) -> Menus {
    println!("--- MACRO MENU ---");
    println!(
        "[1] auto clicker {}",
        if toggle_states.auto_clicker {
            "ON"
        } else {
            "OFF"
        }
    );
    println!(
        "[2] spam E {}",
        if toggle_states.spam_e { "ON" } else { "OFF" }
    );

    // user input
    let input = input();
    let choice = input.trim();

    // auswahl
    match choice {
        "1" => {
            toggle_auto_clicker(toggle_states);
            Menus::MACROS
        }
        "2" => {
            toggle_spam_e(toggle_states);
            Menus::MACROS
        }
        _ => Menus::MAIN,
    }
}

fn toggle_auto_clicker(toggle_states: &mut MacroToggles) {
    toggle_states.auto_clicker = !toggle_states.auto_clicker;
}

fn toggle_spam_e(toggle_states: &mut MacroToggles) {
    toggle_states.spam_e = !toggle_states.spam_e;
}
