use ear_trainer::notes::NoteWithOctave;
use std::io;
use std::time::Duration;

fn main() {
    println!("🎵 Music Note Frequency Calculator 🎵");
    println!("=====================================\n");

    // Quick audio demo
    println!("🎼 Would you like to hear a C major scale demo? (y/n):");
    let mut demo_input = String::new();
    io::stdin()
        .read_line(&mut demo_input)
        .expect("Failed to read line");

    if demo_input.trim().to_lowercase() == "y" {
        println!("🎶 Playing C major scale...\n");
        let scale = ["C4", "D4", "E4", "F4", "G4", "A4", "B4", "C5"];

        for note_str in &scale {
            if let Ok(note) = note_str.parse::<NoteWithOctave>() {
                println!("🎵 Playing {} ({:.1} Hz)", note, note.frequency());
                if let Err(e) = note.play(Duration::from_millis(500)) {
                    println!("❌ Error playing {}: {}", note, e);
                }
            }
        }
        println!("✨ Demo complete!\n");
    }

    // Interactive mode
    println!("🎹 Interactive Mode:");

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
