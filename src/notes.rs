pub const B2: u8 = 71 - 24;

pub const B3: u8 = 71 - 12;
pub const C3: u8 = 60 - 12; // Middle C
pub const CS3: u8 = 61 - 12;
pub const D3: u8 = 62 - 12;
pub const DS3: u8 = 63 - 12;
pub const E3: u8 = 64 - 12;
pub const F3: u8 = 65 - 12;
pub const FS3: u8 = 66 - 12;
pub const G3: u8 = 67 - 12;
pub const GS3: u8 = 68 - 12;
pub const A3: u8 = 69 - 12;

pub const C4: u8 = 60; // Middle C
pub const CS4: u8 = 61;
pub const D4: u8 = 62;
pub const DS4: u8 = 63;
pub const E4: u8 = 64;
pub const F4: u8 = 65;
pub const FS4: u8 = 66;
pub const G4: u8 = 67;
pub const GS4: u8 = 68;
pub const A4: u8 = 69;
pub const AS4: u8 = 70;
pub const B4: u8 = 71;
pub const C5: u8 = 72;

// Helper constants for readability
pub const D5: u8 = 74;
pub const E5: u8 = 76;
pub const F5: u8 = 77;

pub fn note_to_name(note: u8) -> String {
    let note_names = [
        "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
    ];

    let octave = (note / 12) as i8 - 1;
    let note_index = (note % 12) as usize;
    format!("{}{}", note_names[note_index], octave)
}
