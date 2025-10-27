use std::{
    collections::HashMap,
    error::Error,
    io::{Write, stdin, stdout},
};

use evdev::KeyCode;
use midir::MidiInput;

use crate::notes::*;

pub fn setup_midi_mappings() -> HashMap<u8, KeyCode> {
    let mut mappings = HashMap::new();

    // WASD controls (gaming layout)
    mappings.insert(B2, KeyCode::KEY_LEFTCTRL);
    mappings.insert(C3, KeyCode::KEY_LEFTSHIFT);
    mappings.insert(D3, KeyCode::KEY_A);
    mappings.insert(DS3, KeyCode::KEY_W);
    mappings.insert(E3, KeyCode::KEY_S);
    mappings.insert(F3, KeyCode::KEY_D);
    mappings.insert(G3, KeyCode::KEY_SPACE);

    mappings.insert(C4, KeyCode::KEY_X);
    mappings.insert(D4, KeyCode::KEY_Q);

    mappings.insert(FS4, KeyCode::KEY_LEFTCTRL);
    // mappings.insert(G4, KeyCode::KEY_E);
    mappings.insert(GS4, KeyCode::KEY_Q);
    mappings.insert(A4, KeyCode::KEY_R);
    mappings.insert(AS4, KeyCode::KEY_F);
    mappings.insert(B4, KeyCode::KEY_TAB);

    // mappings.insert(C5, KeyCode::KEY_UP);
    // mappings.insert(D5, KeyCode::KEY_DOWN);
    // mappings.insert(E5, KeyCode::KEY_LEFT);
    // mappings.insert(F5, KeyCode::KEY_RIGHT);

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
