use ear_trainer::notes::{Note, NoteWithOctave, Scale, ScaleType, play_frequency};
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 Ear Trainer Audio Demo 🎵");
    println!("============================\n");

    // Demo 1: Single notes
    println!("🎹 Playing individual notes:");
    let notes = ["C4", "E4", "G4"];
    for note_str in &notes {
        let note: NoteWithOctave = note_str.parse()?;
        println!("  Playing {} ({:.1} Hz)", note, note.frequency());
        note.play(Duration::from_millis(800))?;
    }

    println!();

    // Demo 2: Direct frequency playback
    println!("🔊 Playing raw frequencies:");
    let frequencies = [440.0, 523.25, 659.25]; // A4, C5, E5
    for freq in frequencies {
        println!("  Playing {:.1} Hz", freq);
        play_frequency(freq, Duration::from_millis(600))?;
    }

    println!();

    // Demo 3: Scale
    println!("🎵 Playing C major scale:");
    let scale = Scale::new(
        NoteWithOctave {
            note: Note::C,
            octave: 4,
        },
        ScaleType::Major,
    );
    scale.play_default()?;
    println!();

    println!("🎵 Playing F# major scale:");
    let scale = Scale::new(
        NoteWithOctave {
            note: Note::FSharp,
            octave: 4,
        },
        ScaleType::Major,
    );
    scale.play_default()?;
    println!();

    println!("\n✨ Audio demo complete!");
    Ok(())
}
