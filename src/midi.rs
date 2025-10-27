use std::{
    collections::HashMap,
    error::Error,
    io::{Write, stdin, stdout},
};

use evdev::Key;
use midir::MidiInput;

use crate::notes::*;

pub fn setup_midi_mappings() -> HashMap<u8, Key> {
    let mut mappings = HashMap::new();

    // WASD controls (gaming layout)
    mappings.insert(B2, Key::KEY_LEFTCTRL);
    mappings.insert(C3, Key::KEY_LEFTSHIFT);
    mappings.insert(D3, Key::KEY_A);
    mappings.insert(DS3, Key::KEY_W);
    mappings.insert(E3, Key::KEY_S);
    mappings.insert(F3, Key::KEY_D);
    mappings.insert(G3, Key::KEY_SPACE);

    mappings.insert(C4, Key::KEY_X);
    mappings.insert(D4, Key::KEY_Q);

    mappings.insert(FS4, Key::KEY_LEFTCTRL);
    // mappings.insert(G4, Key::KEY_E);
    mappings.insert(GS4, Key::KEY_Q);
    mappings.insert(A4, Key::KEY_R);
    mappings.insert(AS4, Key::KEY_F);
    mappings.insert(B4, Key::KEY_TAB);

    // mappings.insert(C5, Key::KEY_UP);
    // mappings.insert(D5, Key::KEY_DOWN);
    // mappings.insert(E5, Key::KEY_LEFT);
    // mappings.insert(F5, Key::KEY_RIGHT);

    mappings
}

pub fn select_midi_port(midi_in: &MidiInput) -> Result<usize, Box<dyn Error>> {
    let in_ports = midi_in.ports();

    if in_ports.is_empty() {
        return Err("no MIDI input devices found!".into());
    }

    println!("\navailable MIDI input devices:");
    for (i, p) in in_ports.iter().enumerate() {
        println!("  [{}] {}", i, midi_in.port_name(p)?);
    }

    if in_ports.len() == 1 {
        println!("auto-selecting the only available device");
        return Ok(0);
    }

    print!("\nselect device (0-{}): ", in_ports.len() - 1);
    stdout().flush()?;

    let mut input = String::new();
    stdin().read_line(&mut input)?;

    let selection = input.trim().parse::<usize>()?;

    if selection >= in_ports.len() {
        return Err("invalid selection".into());
    }

    Ok(selection)
}
