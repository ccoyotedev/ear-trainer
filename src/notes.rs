use rodio::{OutputStream, Sink, Source};
use std::fmt;
use std::str::FromStr;
use std::time::Duration;

#[derive(Debug, PartialEq)]
pub enum Note {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

#[derive(Debug, PartialEq)]
pub struct NoteWithOctave {
    pub note: Note,
    pub octave: u8,
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let note_str = match self {
            Note::C => "C",
            Note::CSharp => "C#",
            Note::D => "D",
            Note::DSharp => "D#",
            Note::E => "E",
            Note::F => "F",
            Note::FSharp => "F#",
            Note::G => "G",
            Note::GSharp => "G#",
            Note::A => "A",
            Note::ASharp => "A#",
            Note::B => "B",
        };
        write!(f, "{}", note_str)
    }
}

impl fmt::Display for NoteWithOctave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.note, self.octave)
    }
}

impl FromStr for Note {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            // Natural notes
            "C" => Ok(Note::C),
            "D" => Ok(Note::D),
            "E" => Ok(Note::E),
            "F" => Ok(Note::F),
            "G" => Ok(Note::G),
            "A" => Ok(Note::A),
            "B" => Ok(Note::B),

            // Sharp notes
            "C#" => Ok(Note::CSharp),
            "D#" => Ok(Note::DSharp),
            "F#" => Ok(Note::FSharp),
            "G#" => Ok(Note::GSharp),
            "A#" => Ok(Note::ASharp),

            // Flat notes (enharmonic equivalents)
            "Db" => Ok(Note::CSharp),
            "Eb" => Ok(Note::DSharp),
            "Gb" => Ok(Note::FSharp),
            "Ab" => Ok(Note::GSharp),
            "Bb" => Ok(Note::ASharp),

            _ => Err(format!("Invalid note: {}", s)),
        }
    }
}

impl FromStr for NoteWithOctave {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (note_str, octave_str) = match s.find(|c: char| c.is_ascii_digit()) {
            Some(pos) => (&s[..pos], &s[pos..]),
            None => (s, "4"),
        };

        let note = note_str.parse::<Note>()?;
        let octave = octave_str.parse::<u8>().map_err(|e| e.to_string())?;

        Ok(NoteWithOctave { note, octave })
    }
}

impl NoteWithOctave {
    /// Convenience method to get frequency directly from NoteWithOctave
    pub fn frequency(&self) -> f64 {
        self.note.to_frequency(self.octave)
    }

    /// Play the note as audio for the specified duration
    pub fn play(&self, duration: Duration) -> Result<(), Box<dyn std::error::Error>> {
        let frequency = self.frequency() as f32;
        play_frequency(frequency, duration)
    }

    /// Play the note as audio for 1 second (convenience method)
    pub fn play_default(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.play(Duration::from_secs(1))
    }
}

/// Generate a sine wave source at the specified frequency
struct SineWave {
    frequency: f32,
    sample_rate: u32,
    sample_index: usize,
}

impl SineWave {
    fn new(frequency: f32) -> Self {
        Self {
            frequency,
            sample_rate: 44100,
            sample_index: 0,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let sample = (self.sample_index as f32 * self.frequency * 2.0 * std::f32::consts::PI
            / self.sample_rate as f32)
            .sin();
        self.sample_index = self.sample_index.wrapping_add(1);
        Some(sample * 0.3) // Reduce volume to 30%
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

/// Play a frequency for the specified duration
pub fn play_frequency(
    frequency: f32,
    duration: Duration,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create output stream
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // Create sine wave source
    let source = SineWave::new(frequency).take_duration(duration);

    // Play the sound
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

impl Note {
    pub fn to_frequency(&self, octave: u8) -> f64 {
        // A4 = 440Hz as reference
        let semitones_from_a4 = self.semitones_from_a4(octave);
        440.0 * 2.0_f64.powf(semitones_from_a4 as f64 / 12.0)
    }

    /// Convert frequency back to closest note and octave
    pub fn from_frequency(frequency: f64) -> Result<NoteWithOctave, String> {
        if frequency <= 0.0 {
            return Err("Frequency must be positive".to_string());
        }

        // Calculate semitones from A4
        let semitones_from_a4 = 12.0 * (frequency / 440.0).log2();
        let rounded_semitones = semitones_from_a4.round() as i32;

        // Calculate octave (A4 is octave 4)
        let octave = 4 + (rounded_semitones / 12);

        // Handle negative octaves or very high octaves
        if octave < 0 || octave > 10 {
            return Err(format!(
                "Octave {} is out of reasonable range (0-10)",
                octave
            ));
        }

        // Calculate note within octave
        let note_index = ((rounded_semitones % 12) + 12) % 12; // Ensure positive

        let note = match note_index {
            0 => Note::A,
            1 => Note::ASharp,
            2 => Note::B,
            3 => Note::C,
            4 => Note::CSharp,
            5 => Note::D,
            6 => Note::DSharp,
            7 => Note::E,
            8 => Note::F,
            9 => Note::FSharp,
            10 => Note::G,
            11 => Note::GSharp,
            _ => unreachable!("Modulo 12 should only give 0-11"),
        };

        Ok(NoteWithOctave {
            note,
            octave: octave as u8,
        })
    }

    /// Calculate semitones from A4 (440Hz reference)
    /// A4 is our 0 point, so A4 returns 0 semitones
    fn semitones_from_a4(&self, octave: u8) -> i32 {
        // Semitones from A within the same octave
        // Note: C, D, E, F, G come BEFORE A in the same octave number
        let note_offset = match self {
            Note::C => -9, // C is 9 semitones below A
            Note::CSharp => -8,
            Note::D => -7,
            Note::DSharp => -6,
            Note::E => -5,
            Note::F => -4,
            Note::FSharp => -3,
            Note::G => -2,
            Note::GSharp => -1,
            Note::A => 0, // Our reference point
            Note::ASharp => 1,
            Note::B => 2,
        };

        // Calculate octave difference from octave 4
        let octave_offset = (octave as i32 - 4) * 12;

        // Total semitones = octave difference + note offset
        octave_offset + note_offset
    }

    pub fn to_semitone(&self) -> i32 {
        match self {
            Note::C => 0,
            Note::CSharp => 1,
            Note::D => 2,
            Note::DSharp => 3,
            Note::E => 4,
            Note::F => 5,
            Note::FSharp => 6,
            Note::G => 7,
            Note::GSharp => 8,
            Note::A => 9,
            Note::ASharp => 10,
            Note::B => 11,
        }
    }

    pub fn from_semitone(semitone: u8) -> Option<Note> {
        match semitone % 12 {
            0 => Some(Note::C),
            1 => Some(Note::CSharp),
            2 => Some(Note::D),
            3 => Some(Note::DSharp),
            4 => Some(Note::E),
            5 => Some(Note::F),
            6 => Some(Note::FSharp),
            7 => Some(Note::G),
            8 => Some(Note::GSharp),
            9 => Some(Note::A),
            10 => Some(Note::ASharp),
            11 => Some(Note::B),
            _ => None, // Should never happen due to modulo
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScaleType {
    Major,
    Minor,
}

#[derive(Debug, PartialEq)]
pub struct Scale {
    pub root: NoteWithOctave,
    pub scale_type: ScaleType,
}

impl fmt::Display for ScaleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let scale_name = match self {
            ScaleType::Major => "Major",
            ScaleType::Minor => "Minor",
        };
        write!(f, "{}", scale_name)
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.root, self.scale_type)
    }
}

impl FromStr for ScaleType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "major" | "maj" => Ok(ScaleType::Major),
            "minor" | "min" => Ok(ScaleType::Minor),
            _ => Err(format!("Invalid scale type: {}", s)),
        }
    }
}

impl ScaleType {
    pub fn intervals(&self) -> Vec<u8> {
        match self {
            ScaleType::Major => vec![0, 2, 4, 5, 7, 9, 11],
            ScaleType::Minor => vec![0, 2, 3, 5, 7, 8, 10],
        }
    }
}

impl Scale {
    pub fn new(root: NoteWithOctave, scale_type: ScaleType) -> Self {
        Self { root, scale_type }
    }

    pub fn notes(&self) -> Vec<NoteWithOctave> {
        let intervals = self.scale_type.intervals();
        let mut scale_notes = Vec::new();

        for interval in intervals {
            if let Some(note) = self.note_at_interval(interval) {
                scale_notes.push(note);
            }
        }

        scale_notes
    }

    /// Get a specific note at the given interval (in semitones) from the root
    fn note_at_interval(&self, semitones: u8) -> Option<NoteWithOctave> {
        let root_semitone = self.root.note.to_semitone();
        let target_semitone = (root_semitone + semitones as i32) % 12;
        let octave_increase = (root_semitone + semitones as i32) / 12;

        let target_note = Note::from_semitone(target_semitone as u8)?;
        let target_octave = self.root.octave as i32 + octave_increase;

        Some(NoteWithOctave {
            note: target_note,
            octave: target_octave as u8,
        })
    }

    pub fn play(&self, note_duration: Duration) -> Result<(), Box<dyn std::error::Error>> {
        let notes = self.notes();
        for note in notes {
            println!("🎵 {}", note);
            note.play(note_duration)?;
        }
        Ok(())
    }

    /// Play the scale with default timing (500ms per note)
    pub fn play_default(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.play(Duration::from_millis(500))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a4_is_440hz() {
        let a4_freq = Note::A.to_frequency(4);
        assert!(
            (a4_freq - 440.0).abs() < 0.01,
            "A4 should be 440Hz, got {}",
            a4_freq
        );
    }

    #[test]
    fn test_c4_frequency() {
        let c4_freq = Note::C.to_frequency(4);
        // C4 should be approximately 261.63Hz
        assert!(
            (c4_freq - 261.63).abs() < 0.01,
            "C4 should be ~261.63Hz, got {}",
            c4_freq
        );
    }

    #[test]
    fn test_octave_doubling() {
        let a4_freq = Note::A.to_frequency(4);
        let a5_freq = Note::A.to_frequency(5);

        // A5 should be exactly double A4
        assert!(
            (a5_freq - a4_freq * 2.0).abs() < 0.01,
            "A5 ({}) should be double A4 ({})",
            a5_freq,
            a4_freq
        );
    }

    #[test]
    fn test_semitone_relationship() {
        let c4_freq = Note::C.to_frequency(4);
        let c_sharp4_freq = Note::CSharp.to_frequency(4);

        // Each semitone should multiply frequency by 2^(1/12) ≈ 1.0595
        let expected_ratio = 2.0_f64.powf(1.0 / 12.0);
        let actual_ratio = c_sharp4_freq / c4_freq;

        assert!(
            (actual_ratio - expected_ratio).abs() < 0.001,
            "Semitone ratio should be ~1.0595, got {}",
            actual_ratio
        );
    }

    #[test]
    fn test_frequency_to_note_a4() {
        let note = Note::from_frequency(440.0).unwrap();
        assert_eq!(note.note, Note::A);
        assert_eq!(note.octave, 4);
    }

    #[test]
    fn test_frequency_to_note_c4() {
        let c4_freq = Note::C.to_frequency(4);
        let note = Note::from_frequency(c4_freq).unwrap();
        assert_eq!(note.note, Note::C);
        assert_eq!(note.octave, 4);
    }

    #[test]
    fn test_round_trip_conversion() {
        // Test that converting note->frequency->note gives us back the original
        let original_notes = vec![(Note::C, 4), (Note::FSharp, 3), (Note::A, 4), (Note::B, 5)];

        for (note, octave) in original_notes {
            let freq = note.to_frequency(octave);
            let converted = Note::from_frequency(freq).unwrap();

            assert_eq!(
                converted.note, note,
                "Round trip failed for {:?}{}",
                note, octave
            );
            assert_eq!(
                converted.octave, octave,
                "Octave mismatch for {:?}{}",
                note, octave
            );
        }
    }

    #[test]
    fn test_frequency_error_handling() {
        // Test negative frequency
        assert!(Note::from_frequency(-1.0).is_err());

        // Test zero frequency
        assert!(Note::from_frequency(0.0).is_err());

        // Test extremely high frequency (should be out of range)
        assert!(Note::from_frequency(100000.0).is_err());
    }

    #[test]
    fn test_display_formatting() {
        assert_eq!(format!("{}", Note::C), "C");
        assert_eq!(format!("{}", Note::CSharp), "C#");
        assert_eq!(format!("{}", Note::FSharp), "F#");

        let note_with_octave = NoteWithOctave {
            note: Note::A,
            octave: 4,
        };
        assert_eq!(format!("{}", note_with_octave), "A4");
    }

    #[test]
    fn test_note_with_octave_convenience() {
        let note = NoteWithOctave {
            note: Note::A,
            octave: 4,
        };
        assert!((note.frequency() - 440.0).abs() < 0.01);
    }

    #[test]
    fn test_note_with_octave_from_str() {
        let note = NoteWithOctave::from_str("A5").unwrap();
        assert_eq!(note.note, Note::A);
        assert_eq!(note.octave, 5);
    }

    #[test]
    fn test_note_with_octave_from_str_with_sharp() {
        let note = NoteWithOctave::from_str("A#2").unwrap();
        assert_eq!(note.note, Note::ASharp);
        assert_eq!(note.octave, 2);
    }

    #[test]
    fn test_note_with_octave_from_str_with_flat() {
        let note = NoteWithOctave::from_str("Bb3").unwrap();
        assert_eq!(note.note, Note::ASharp);
        assert_eq!(note.octave, 3);
    }

    #[test]
    fn test_note_with_octave_from_str_no_octave() {
        let note = NoteWithOctave::from_str("C").unwrap();
        assert_eq!(note.note, Note::C);
        assert_eq!(note.octave, 4);
    }

    #[test]
    fn test_note_with_octave_from_str_no_octave_with_sharp() {
        let note = NoteWithOctave::from_str("C#").unwrap();
        assert_eq!(note.note, Note::CSharp);
        assert_eq!(note.octave, 4);
    }

    #[test]
    fn test_note_with_octave_from_str_no_octave_with_flat() {
        let note = NoteWithOctave::from_str("Db").unwrap();
        assert_eq!(note.note, Note::CSharp);
        assert_eq!(note.octave, 4);
    }

    #[test]
    fn test_note_with_octave_from_str_invalid() {
        assert!(NoteWithOctave::from_str("Cb").is_err());
        assert!(NoteWithOctave::from_str("B#").is_err());
        assert!(NoteWithOctave::from_str("foo").is_err());
        assert!(NoteWithOctave::from_str("Dl").is_err());
        assert!(NoteWithOctave::from_str("Gfoobar2").is_err());
        assert!(NoteWithOctave::from_str("2").is_err());
    }

    #[test]
    fn test_scale_type_parsing() {
        assert_eq!("major".parse::<ScaleType>().unwrap(), ScaleType::Major);
        assert_eq!("min".parse::<ScaleType>().unwrap(), ScaleType::Minor);
        assert!("invalid".parse::<ScaleType>().is_err());
    }

    #[test]
    fn test_scale_intervals() {
        let major_intervals = ScaleType::Major.intervals();
        assert_eq!(major_intervals, vec![0, 2, 4, 5, 7, 9, 11]);

        let minor_intervals = ScaleType::Minor.intervals();
        assert_eq!(minor_intervals, vec![0, 2, 3, 5, 7, 8, 10]);
    }

    #[test]
    fn test_c_major_scale() {
        let scale = Scale::new(
            NoteWithOctave {
                note: Note::C,
                octave: 4,
            },
            ScaleType::Major,
        );
        let notes = scale.notes();

        assert_eq!(notes.len(), 7);
        assert_eq!(notes[0].note, Note::C);
        assert_eq!(notes[1].note, Note::D);
        assert_eq!(notes[2].note, Note::E);
        assert_eq!(notes[3].note, Note::F);
        assert_eq!(notes[4].note, Note::G);
        assert_eq!(notes[5].note, Note::A);
        assert_eq!(notes[6].note, Note::B);
    }

    #[test]
    fn test_scale_display() {
        let scale = Scale::new(
            NoteWithOctave {
                note: Note::A,
                octave: 4,
            },
            ScaleType::Minor,
        );
        assert_eq!(format!("{}", scale), "A4 Minor");
    }

    #[test]
    fn test_note_semitone_conversion() {
        assert_eq!(Note::C.to_semitone(), 0);
        assert_eq!(Note::CSharp.to_semitone(), 1);
        assert_eq!(Note::B.to_semitone(), 11);

        assert_eq!(Note::from_semitone(0).unwrap(), Note::C);
        assert_eq!(Note::from_semitone(1).unwrap(), Note::CSharp);
        assert_eq!(Note::from_semitone(11).unwrap(), Note::B);
        assert_eq!(Note::from_semitone(12).unwrap(), Note::C); // Wraps around
    }
}
