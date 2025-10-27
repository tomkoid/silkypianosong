use std::collections::HashMap;

use evdev::KeyCode;

use crate::notes::note_to_name;

pub fn print_mappings(mappings: &HashMap<u8, KeyCode>) {
    println!("\nmappings:");
    println!("┌────────────┬──────────────┐");
    println!("│ MIDI Note  │ Keyboard Key │");
    println!("├────────────┼──────────────┤");

    let mut sorted_mappings: Vec<_> = mappings.iter().collect();
    sorted_mappings.sort_by_key(|(note, _)| *note);

    for (note, key) in sorted_mappings {
        let key_name = format!("{:?}", key).replace("KEY_", "");
        println!(
            "│ {:4} ({:3}) │ {:12} │",
            note,
            note_to_name(*note),
            key_name
        );
    }
    println!("└────────────┴──────────────┘\n");
}
