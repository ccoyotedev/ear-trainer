# 🎵 Ear Trainer - Music Note Frequency Calculator

A Rust-based music note frequency calculator with audio playback capabilities. Perfect for learning both Rust programming concepts and music theory!

## 🎯 Features

- **Bidirectional Conversion**: Convert notes to frequencies and frequencies back to notes
- **Audio Playback**: Hear the actual notes through your speakers
- **Interactive CLI**: User-friendly command-line interface
- **Comprehensive Testing**: Full test coverage with 17+ tests
- **Musical Accuracy**: Uses equal temperament tuning (A4 = 440Hz)
- **Error Handling**: Robust input validation and error messages

## 🎼 Supported Note Formats

- **Natural notes**: `C`, `D`, `E`, `F`, `G`, `A`, `B`
- **Sharp notes**: `C#`, `D#`, `F#`, `G#`, `A#`
- **Flat notes**: `Db`, `Eb`, `Gb`, `Ab`, `Bb`
- **Octave notation**: `C4`, `A#3`, `Bb2` (defaults to octave 4 if omitted)

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+ installed
- Audio output device

### Installation & Running

```bash
# Clone and navigate to the project
cd ear_trainer

# Run the interactive calculator
cargo run

# Run the audio demo
cargo run --example audio_demo

# Run tests
cargo test
```

## 🎹 Usage Examples

### Interactive Mode

```
🎵 Enter a note (e.g. C4, A#3, Bb2) or 'q' to quit:
A4
📊 A4 = 440.00 Hz
🔊 Play this note? (y/n/duration in seconds):
y
🎶 Playing A4 for 1 second...
```

### Programming API

```rust
use ear_trainer::notes::NoteWithOctave;
use std::time::Duration;

// Parse a note from string
let note: NoteWithOctave = "C4".parse()?;

// Get frequency
println!("{} = {:.2} Hz", note, note.frequency()); // C4 = 261.63 Hz

// Play the note
note.play(Duration::from_secs(1))?;

// Convert frequency back to note
let note = Note::from_frequency(440.0)?; // Returns A4
```

## 🦀 Rust Concepts Demonstrated

This project showcases many important Rust concepts:

### Core Language Features

- **Enums**: Musical notes represented as enum variants
- **Structs**: `NoteWithOctave` combining note and octave data
- **Pattern Matching**: Exhaustive `match` statements for note parsing
- **Traits**: `Display`, `FromStr`, `Debug`, `PartialEq`
- **Modules**: Organized code structure with `src/notes.rs`

### Error Handling

- **Result Types**: `Result<T, E>` for robust error handling
- **Custom Errors**: Meaningful error messages for invalid input
- **Error Propagation**: Using `?` operator effectively

### Testing & Quality

- **Unit Tests**: Comprehensive test suite with `#[test]`
- **Test Organization**: Tests in dedicated module with `#[cfg(test)]`
- **Documentation**: Clear examples and API documentation

### Audio Programming

- **External Crates**: Integration with `rodio` for audio
- **Iterator Traits**: Custom `SineWave` implementing `Iterator`
- **Source Trait**: Audio source implementation
- **Duration Handling**: Time-based audio control

## 🎼 Musical Theory

The calculator implements **equal temperament tuning**:

- **Reference**: A4 = 440Hz (international standard)
- **Semitone ratio**: 2^(1/12) ≈ 1.0595
- **Octave relationship**: Perfect 2:1 frequency ratio
- **12-tone system**: All chromatic notes supported

### Frequency Formula

```
frequency = 440 × 2^(semitones_from_A4 / 12)
```

## 📁 Project Structure

```
ear_trainer/
├── src/
│   ├── lib.rs          # Library root
│   ├── main.rs         # Interactive CLI application
│   └── notes.rs        # Core note and audio functionality
├── examples/
│   └── audio_demo.rs   # Audio demonstration
├── Cargo.toml          # Dependencies and metadata
└── README.md           # This file
```

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

Tests include:

- Note to frequency conversion accuracy
- Frequency to note conversion
- Round-trip conversion verification
- Error handling edge cases
- String parsing validation
- Musical relationship verification (octaves, semitones)

## 🔧 Dependencies

- `rodio = "0.17"` - Cross-platform audio library

## 🎯 Future Extensions

This project provides a solid foundation for:

- **Interactive ear training games**
- **Guitar/piano tuning tools**
- **Music theory education software**
- **Audio synthesis experiments**
- **Web API development** (with `axum`/`warp`)
- **GUI applications** (with `egui`/`tauri`)

## 🎵 Example Output

```
🎵 Playing C major scale:
C4 D4 E4 F4 G4 A4 B4 C5

🎼 Playing C major chord (arpeggio):
  C4 (261.6 Hz)
  E4 (329.6 Hz)
  G4 (392.0 Hz)
```

## 📚 Learning Resources

This project demonstrates practical applications of:

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Music Theory Fundamentals](https://en.wikipedia.org/wiki/Equal_temperament)
- [Audio Programming in Rust](https://docs.rs/rodio/)

---

**Happy coding and happy music-making!** 🎵✨
