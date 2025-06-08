use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Note {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B
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

impl NoteWithOctave {
    /// Convenience method to get frequency directly from NoteWithOctave
    pub fn frequency(&self) -> f64 {
        self.note.to_frequency(self.octave)
    }
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
            return Err(format!("Octave {} is out of reasonable range (0-10)", octave));
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
            Note::C => -9,      // C is 9 semitones below A
            Note::CSharp => -8,
            Note::D => -7,
            Note::DSharp => -6,
            Note::E => -5,
            Note::F => -4,
            Note::FSharp => -3,
            Note::G => -2,
            Note::GSharp => -1,
            Note::A => 0,       // Our reference point
            Note::ASharp => 1,
            Note::B => 2,
        };

        // Calculate octave difference from octave 4
        let octave_offset = (octave as i32 - 4) * 12;
        
        // Total semitones = octave difference + note offset
        octave_offset + note_offset
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a4_is_440hz() {
        let a4_freq = Note::A.to_frequency(4);
        assert!((a4_freq - 440.0).abs() < 0.01, "A4 should be 440Hz, got {}", a4_freq);
    }

    #[test]
    fn test_c4_frequency() {
        let c4_freq = Note::C.to_frequency(4);
        // C4 should be approximately 261.63Hz
        assert!((c4_freq - 261.63).abs() < 0.01, "C4 should be ~261.63Hz, got {}", c4_freq);
    }

    #[test]
    fn test_octave_doubling() {
        let a4_freq = Note::A.to_frequency(4);
        let a5_freq = Note::A.to_frequency(5);
        
        // A5 should be exactly double A4
        assert!((a5_freq - a4_freq * 2.0).abs() < 0.01, 
            "A5 ({}) should be double A4 ({})", a5_freq, a4_freq);
    }

    #[test]
    fn test_semitone_relationship() {
        let c4_freq = Note::C.to_frequency(4);
        let c_sharp4_freq = Note::CSharp.to_frequency(4);
        
        // Each semitone should multiply frequency by 2^(1/12) ≈ 1.0595
        let expected_ratio = 2.0_f64.powf(1.0/12.0);
        let actual_ratio = c_sharp4_freq / c4_freq;
        
        assert!((actual_ratio - expected_ratio).abs() < 0.001,
            "Semitone ratio should be ~1.0595, got {}", actual_ratio);
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
        let original_notes = vec![
            (Note::C, 4),
            (Note::FSharp, 3),
            (Note::A, 4),
            (Note::B, 5),
        ];

        for (note, octave) in original_notes {
            let freq = note.to_frequency(octave);
            let converted = Note::from_frequency(freq).unwrap();
            
            assert_eq!(converted.note, note, 
                "Round trip failed for {:?}{}", note, octave);
            assert_eq!(converted.octave, octave,
                "Octave mismatch for {:?}{}", note, octave);
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
        
        let note_with_octave = NoteWithOctave { note: Note::A, octave: 4 };
        assert_eq!(format!("{}", note_with_octave), "A4");
    }

    #[test]
    fn test_note_with_octave_convenience() {
        let note = NoteWithOctave { note: Note::A, octave: 4 };
        assert!((note.frequency() - 440.0).abs() < 0.01);
    }
}