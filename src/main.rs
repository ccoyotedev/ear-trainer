use ear_trainer::notes::{NoteWithOctave};
use std::io;

fn main() {
    println!("🎵 Music Note Frequency Calculator 🎵");
    println!("=====================================\n");

    loop {
        println!("Enter a note (e.g. C4, A#3, Bb2) or 'q' to quit:");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input == "q" {
            break;
        }

        let note = match input.parse::<NoteWithOctave>() {
            Ok(note) => note,
            Err(_) => {
                println!("Invalid input. Please enter a valid note (e.g. C4, A#3, Bb2).");
                continue;
            }
        };

        println!("Frequency of {}: {:.2} Hz", note, note.frequency());
    }
}
