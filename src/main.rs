use evdev::{AttributeSet, EventType, InputEvent, KeyCode, uinput::VirtualDeviceBuilder};
use midir::{Ignore, MidiInput};
use std::collections::HashMap;
use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use crate::helpers::print_mappings;
use crate::midi::{select_midi_port, setup_midi_mappings};
use crate::notes::note_to_name;

mod helpers;
mod midi;
mod notes;

fn main() -> Result<(), Box<dyn Error>> {
    // check if running with perms
    if std::fs::metadata("/dev/uinput").is_err() {
        eprintln!("cannot access /dev/uinput");
        eprintln!("   run with: sudo {}", std::env::args().next().unwrap());
        eprintln!("   or add your user to input group:");
        eprintln!("   sudo usermod -aG input $USER");
        return Err("insufficient permissions".into());
    }

    let device = Arc::new(Mutex::new(create_virtual_keyboard()?));

    let mappings = Arc::new(setup_midi_mappings());
    print_mappings(&mappings);

    let pressed_notes: Arc<Mutex<HashMap<u8, bool>>> = Arc::new(Mutex::new(HashMap::new()));

    let mut midi_in = MidiInput::new("MIDI to KeyCodeboard")?;
    midi_in.ignore(Ignore::None);

    // select input port
    let port_index = select_midi_port(&midi_in)?;
    let in_ports = midi_in.ports();
    let in_port = &in_ports[port_index];
    let port_name = midi_in.port_name(in_port)?;

    println!("ronnected to: {}", port_name);
    println!("listening for MIDI input... (press Ctrl+C to exit)\n");

    // setup exit handler
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })?;

    // clone for callback
    let device_clone = device.clone();
    let mappings_clone = mappings.clone();
    let pressed_notes_clone = pressed_notes.clone();

    // connect to MIDI
    let _conn = midi_in.connect(
        in_port,
        "midi-input",
        move |_timestamp, message, _| {
            if message.len() < 3 {
                return;
            }

            let status = message[0] & 0xF0;
            let note = message[1];
            let velocity = message[2];

            let is_note_on = status == 0x90 && velocity > 0;
            let is_note_off = status == 0x80 || (status == 0x90 && velocity == 0);

            if let Some(&key) = mappings_clone.get(&note) {
                let mut device = device_clone.lock().unwrap();
                let mut pressed = pressed_notes_clone.lock().unwrap();

                if is_note_on && !pressed.get(&note).copied().unwrap_or(false) {
                    pressed.insert(note, true);
                    let key_name = format!("{:?}", key).replace("KEY_", "");
                    println!("{} ({}) → {} DOWN", note_to_name(note), note, key_name);

                    let _ = device.emit(&[InputEvent::new(1, key.code(), 1)]);
                } else if is_note_off {
                    pressed.insert(note, false);
                    let key_name = format!("{:?}", key).replace("KEY_", "");
                    println!("{} ({}) → {} UP", note_to_name(note), note, key_name);

                    let _ = device.emit(&[InputEvent::new(1, key.code(), 0)]);
                }
            } else if is_note_on {
                println!("unmapped note: {} ({})", note_to_name(note), note);
            }
        },
        (),
    )?;

    // Keep running
    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    // release all pressed keys
    println!("shutting down...");
    let mut device = device.lock().unwrap();
    let pressed = pressed_notes.lock().unwrap();

    for (note, is_pressed) in pressed.iter() {
        if *is_pressed && let Some(&key) = mappings.get(note) {
            let _ = device.emit(&[InputEvent::new(1, key.code(), 0)]);
        }
    }

    Ok(())
}

fn create_virtual_keyboard() -> Result<evdev::uinput::VirtualDevice, Box<dyn Error>> {
    let mut keys = AttributeSet::<KeyCode>::new();

    // all the keys we wanna use
    keys.insert(KeyCode::KEY_W);
    keys.insert(KeyCode::KEY_A);
    keys.insert(KeyCode::KEY_S);
    keys.insert(KeyCode::KEY_D);
    keys.insert(KeyCode::KEY_Q);
    keys.insert(KeyCode::KEY_E);
    keys.insert(KeyCode::KEY_R);
    keys.insert(KeyCode::KEY_F);
    keys.insert(KeyCode::KEY_X);
    keys.insert(KeyCode::KEY_SPACE);
    keys.insert(KeyCode::KEY_LEFTSHIFT);
    keys.insert(KeyCode::KEY_LEFTCTRL);
    keys.insert(KeyCode::KEY_TAB);
    keys.insert(KeyCode::KEY_1);
    keys.insert(KeyCode::KEY_2);
    keys.insert(KeyCode::KEY_3);
    keys.insert(KeyCode::KEY_4);
    keys.insert(KeyCode::KEY_5);
    keys.insert(KeyCode::KEY_UP);
    keys.insert(KeyCode::KEY_DOWN);
    keys.insert(KeyCode::KEY_LEFT);
    keys.insert(KeyCode::KEY_RIGHT);
    keys.insert(KeyCode::KEY_ENTER);
    keys.insert(KeyCode::KEY_ESC);

    let device = VirtualDeviceBuilder::new()?
        .name("MIDI Virtual KeyCodeboard")
        .with_keys(&keys)?
        .build()?;

    println!("virtual keyboard device created");
    Ok(device)
}
