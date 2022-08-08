use std::{fmt, ops::Deref};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Vowels {
    NearOpenFrontUroundedVowel
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Consonants {
    VoicedAlveolarNasal
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Sound {
    Vowel { phoneme: Vowels, is_long: bool },
    Consonant { phoneme: Consonants, is_long: bool, is_palatalized: bool }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SoundError {
    NotPalatalized(Consonants)
}

impl fmt::Debug for SoundError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SoundError::NotPalatalized(consonant) => {
                write!(formatter, "{:?} must be palatalized", consonant)?;
            }
        }
        Ok(())
    }
}

impl Sound {
    pub fn new(phoneme: Sound) -> Result<Self, SoundError> {
        match phoneme {
            Sound::Vowel { phoneme, is_long } => Ok(Sound::Vowel { phoneme, is_long }),
            Sound::Consonant { phoneme, is_long, is_palatalized } => match phoneme {
                _ => Ok(Sound::Consonant { phoneme, is_long, is_palatalized })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ipa(Vec<Sound>);

impl Ipa {
    pub fn new(ipa: &str) -> Result<Self, SoundError> {
        let ipa: Vec<_> = ipa.chars().collect();
        let mut vec = Vec::with_capacity(ipa.len());
        for i in 0..ipa.len() {
            let is_palatalizizg_next = if i == ipa.len() - 1 {
                false
            } else {
                match ipa[i + 1] {
                    'ʲ' => true,
                    _ => false
                }
            };
            match ipa[i] {
                'n' => vec.push(Sound::new(Sound::Consonant {
                    phoneme: Consonants::VoicedAlveolarNasal,
                    is_long: false,
                    is_palatalized: is_palatalizizg_next
                })?),
                'æ' => vec.push(Sound::new(Sound::Vowel {
                    phoneme: Vowels::NearOpenFrontUroundedVowel,
                    is_long: false,
                })?),
                'ʲ' => continue,
                _ => todo!()
            }
        }
        Ok(Ipa(vec))
    }
}

impl Deref for Ipa {
    type Target = [Sound];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod sound_build_tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(
            Sound::new(Sound::Vowel { phoneme: Vowels::NearOpenFrontUroundedVowel, is_long: false }),
            Ok(Sound::Vowel { phoneme: Vowels::NearOpenFrontUroundedVowel, is_long: false })
        );
        assert_eq!(
            Sound::new(Sound::Vowel { phoneme: Vowels::NearOpenFrontUroundedVowel, is_long: true }),
            Ok(Sound::Vowel { phoneme: Vowels::NearOpenFrontUroundedVowel, is_long: true })
        );
        assert_eq!(
            Sound::new(Sound::Consonant { phoneme: Consonants::VoicedAlveolarNasal, is_long: true, is_palatalized: false }),
            Ok(Sound::Consonant { phoneme: Consonants::VoicedAlveolarNasal, is_long: true, is_palatalized: false })
        );
        assert_eq!(
            Sound::new(Sound::Consonant { phoneme: Consonants::VoicedAlveolarNasal, is_long: false, is_palatalized: true }),
            Ok(Sound::Consonant { phoneme: Consonants::VoicedAlveolarNasal, is_long: false, is_palatalized: true })
        );
    }
}

#[cfg(test)]
mod ipa_build_tests {
    use super::*;

    #[test]
    fn test_nja() {
        assert_eq!(
            Ipa::new("nʲæ"),
            Ok(Ipa(vec![
                Sound::Consonant {
                    phoneme: Consonants::VoicedAlveolarNasal,
                    is_long: false,
                    is_palatalized: true
                },
                Sound::Vowel {
                    phoneme: Vowels::NearOpenFrontUroundedVowel,
                    is_long: false
                }
            ]))
        );
        
    }
}
