use ear_trainer::notes::{Note, NoteWithOctave};

fn main() {
    println!("🎵 Music Note Frequency Calculator 🎵");
    println!("=====================================\n");

    // Demo 1: Note to Frequency
    println!("📈 Note to Frequency Conversion:");
    let demo_notes = vec![
        (Note::A, 4),
        (Note::C, 4),
        (Note::E, 4),
        (Note::G, 4),
        (Note::C, 5),
    ];

    for (note, octave) in demo_notes {
        let freq = note.to_frequency(octave);
        println!("  {} = {:.2} Hz", 
            NoteWithOctave { note, octave }, freq);
    }

    println!();

    // Demo 2: Frequency to Note
    println!("📉 Frequency to Note Conversion:");
    let demo_frequencies = vec![440.0, 261.63, 329.63, 392.0, 523.25];
    
    for freq in demo_frequencies {
        match Note::from_frequency(freq) {
            Ok(note) => println!("  {:.2} Hz = {}", freq, note),
            Err(e) => println!("  {:.2} Hz = Error: {}", freq, e),
        }
    }

    println!();

    // Demo 3: Musical Relationships
    println!("🎼 Musical Relationships:");
    
    // Show octave doubling
    let a4_freq = Note::A.to_frequency(4);
    let a5_freq = Note::A.to_frequency(5);
    println!("  Octave doubling: A4 ({:.1} Hz) → A5 ({:.1} Hz) = {:.1}x", 
        a4_freq, a5_freq, a5_freq / a4_freq);

    // Show semitone ratio
    let c4_freq = Note::C.to_frequency(4);
    let c_sharp4_freq = Note::CSharp.to_frequency(4);
    let semitone_ratio = c_sharp4_freq / c4_freq;
    println!("  Semitone ratio: C4 → C#4 = {:.4}x (≈ 2^(1/12))", semitone_ratio);

    // Show perfect fifth
    let c4_freq = Note::C.to_frequency(4);
    let g4_freq = Note::G.to_frequency(4);
    let fifth_ratio = g4_freq / c4_freq;
    println!("  Perfect fifth: C4 → G4 = {:.4}x (≈ 1.5)", fifth_ratio);

    println!();

    // Demo 4: Error Handling
    println!("⚠️  Error Handling:");
    match Note::from_frequency(-100.0) {
        Ok(_) => println!("  This shouldn't happen!"),
        Err(e) => println!("  Negative frequency: {}", e),
    }

    match Note::from_frequency(100000.0) {
        Ok(_) => println!("  This shouldn't happen!"),
        Err(e) => println!("  Too high frequency: {}", e),
    }

    println!("\n🎯 Ready to tune your instruments!");
}
