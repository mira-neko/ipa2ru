use std::{fmt, ops::Deref};

#[deny(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Vowels {
    CloseBackRoundedVowel,
    NearOpenFrontUroundedVowel,
}

#[deny(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Consonants {
    VoicedAlveolarNasal,
    VoicedBilabialNasal
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Sound {
    Vowel { phoneme: Vowels, is_long: bool },
    Consonant { phoneme: Consonants, is_long: bool, is_palatalized: bool }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ipa(Vec<Sound>);

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Error {
    PalatalizedVowel(Vowels)
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PalatalizedVowel(vowel) => {
                write!(formatter, "Vowel ({:?}) cannot be palatalized", vowel)?;
            }
        }
        Ok(())
    }
}

macro_rules! push_sound {
    ($vec:ident, $phoneme:expr) => {
        $vec.push($phoneme)
    };
}

macro_rules! push_vowel {
    ($vec:ident, $is_long:ident, $is_palatalizizg_next:ident, $phoneme:ident) => {
        if $is_palatalizizg_next {
            return Err(Error::PalatalizedVowel(Vowels::$phoneme))
        } else {
            push_sound!($vec, Sound::Vowel {
                phoneme: Vowels::$phoneme,
                is_long: $is_long,
            })
        }
    };
}

macro_rules! push_consonant {
    ($vec:ident, $is_long:ident, $is_palatalizizg_next:ident, $phoneme:ident) => {
        push_sound!($vec, Sound::Consonant {
            phoneme: Consonants::$phoneme,
            is_long: $is_long,
            is_palatalized: $is_palatalizizg_next
        })
    };
}

impl Ipa {
    pub fn new(ipa: &str) -> Result<Self, Error> {
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
            let is_longing_next = if i == ipa.len() - 1 {
                false
            } else if i < ipa.len() - 2 && is_palatalizizg_next {
                match ipa[i + 2] {
                    'ː' => true,
                    _ => false
                }
            } else {
                match ipa[i + 1] {
                    'ː' => true,
                    _ => false
                }
            };
            match ipa[i] {
                'n' => push_consonant!(vec, is_longing_next, is_palatalizizg_next, VoicedAlveolarNasal),
                'm' => push_consonant!(vec, is_longing_next, is_palatalizizg_next, VoicedBilabialNasal),

                'u' => push_vowel!(vec, is_longing_next, is_palatalizizg_next, CloseBackRoundedVowel),
                'æ' => push_vowel!(vec, is_longing_next, is_palatalizizg_next, NearOpenFrontUroundedVowel),

                'ʲ' => continue,
                'ː' => continue,
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

    #[test]
    fn test_palatalized_vowel() {
        assert_eq!(
            Ipa::new("æʲ"),
            Err(Error::PalatalizedVowel(Vowels::NearOpenFrontUroundedVowel))
        );
        
    }
}
