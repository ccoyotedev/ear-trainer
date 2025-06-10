use ear_trainer::notes::{NoteWithOctave, Scale, ScaleType};
use std::io;

fn main() {
    println!("🎵 Music Note Frequency Calculator 🎵");
    println!("=====================================\n");

    loop {
        println!("Play a note or a scale? [n/s]");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "n" {
            handle_note_path();
            break;
        }

        if input == "s" {
            handle_scale_path();
            break;
        }
    }

    fn handle_note_path() {
        loop {
            println!("\n🎵 Enter a note (e.g. C4, A#3, Bb2) or 'q' to quit:");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let input = input.trim();

            if input == "q" {
                println!("👋 Goodbye!");
                break;
            }

            let note = match input.parse::<NoteWithOctave>() {
                Ok(note) => note,
                Err(_) => {
                    println!("❌ Invalid input. Please enter a valid note (e.g. C4, A#3, Bb2).");
                    continue;
                }
            };

            println!("📊 {} = {:.2} Hz", note, note.frequency());

            match note.play_default() {
                Ok(_) => println!("🎶 Playing {}...", note),
                Err(e) => println!("❌ Error playing {}: {}", note, e),
            }
        }
    }

    fn handle_scale_path() {
        loop {
            println!("\n🎵 Enter a scale (e.g. C major, A minor, F# major):");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            let (note, scale_type) = match input.split_once(" ") {
                Some((note, scale_type)) => (note, scale_type.trim()),
                None => {
                    println!(
                        "❌ Invalid input. Please enter a valid scale (e.g. C major, A minor, F# major)."
                    );
                    continue;
                }
            };

            let note = match note.parse::<NoteWithOctave>() {
                Ok(note) => note,
                Err(_) => {
                    println!("❌ Invalid input. Please enter a valid note (e.g. C4, A#3, Bb2).");
                    continue;
                }
            };

            let scale_type = match scale_type.parse::<ScaleType>() {
                Ok(scale_type) => scale_type,
                Err(_) => {
                    println!(
                        "❌ Invalid input. Please enter a valid scale type (e.g. major, minor)."
                    );
                    continue;
                }
            };

            let scale = Scale::new(note, scale_type);

            match scale.play_default() {
                Ok(_) => println!("🎶 Playing {}...", scale),
                Err(e) => println!("❌ Error playing {}: {}", scale, e),
            }
        }
    }
}
