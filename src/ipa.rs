use std::{fmt, ops::Deref};

#[deny(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Vowels {
    CloseBackRounded,
    CloseMidFrontRounded,
    MidCentral,
    NearOpenFrontUrounded,
    OpenBackUnrounded,
    OpenFrontUnrounded,
    OpenMidBackUnrounded
}

#[deny(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
pub enum Consonants {
    VoicedAlveolarNasal,
    VoicedBilabialNasal,
    VoicedPalatalApproximant,
    VoicelessBilabialPlosive
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
    PalatalizedVowel(char)
}

impl fmt::Debug for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::PalatalizedVowel(vowel) => {
                write!(formatter, "Vowel ({}) cannot be palatalized", vowel)?;
            }
        }
        Ok(())
    }
}

impl Ipa {
    pub fn new(ipa: &str) -> Result<Self, Error> {
        use Sound::*;
        use Consonants::*;
        use Vowels::*;
        use Error::*;

        let ipa: Vec<_> = ipa.chars().collect();
        let mut vec = Vec::with_capacity(ipa.len());
        for i in 0..ipa.len() {
            let is_palatalized = if i == ipa.len() - 1 {
                false
            } else {
                matches!(ipa[i + 1], 'ʲ')
            };
            let is_long = if i == ipa.len() - 1 {
                false
            } else if i < ipa.len() - 2 && is_palatalized {
                matches!(ipa[i + 2], 'ː')
            } else {
                matches!(ipa[i + 1], 'ː')
            };
            let sound = match ipa[i] {
                'n' => Some(Consonant { phoneme: VoicedAlveolarNasal,      is_long, is_palatalized }),
                'm' => Some(Consonant { phoneme: VoicedBilabialNasal,      is_long, is_palatalized }),
                'j' => Some(Consonant { phoneme: VoicedPalatalApproximant, is_long, is_palatalized }),
                'p' => Some(Consonant { phoneme: VoicelessBilabialPlosive, is_long, is_palatalized }),

                'u' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: CloseBackRounded,      is_long }) },
                'ø' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: CloseMidFrontRounded,  is_long }) },
                'ə' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: MidCentral,            is_long }) },
                'æ' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: NearOpenFrontUrounded, is_long }) },
                'ɑ' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: OpenBackUnrounded,     is_long }) },
                'a' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: OpenFrontUnrounded,    is_long }) },
                'ʌ' => if is_palatalized { return Err(PalatalizedVowel(ipa[i])); } else { Some(Vowel { phoneme: OpenMidBackUnrounded,  is_long }) },

                'ʲ' => None,
                'ː' => None,
                _ => todo!()
            };
            if let Some(to_push) = sound {
                vec.push(to_push);
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
    fn test_na() {
        assert_eq!(
            Ipa::new("nʲæ"),
            Ok(Ipa(vec![
                Sound::Consonant {
                    phoneme: Consonants::VoicedAlveolarNasal,
                    is_long: false,
                    is_palatalized: true
                },
                Sound::Vowel {
                    phoneme: Vowels::NearOpenFrontUrounded,
                    is_long: false
                }
            ]))
        );
        
    }

    #[test]
    fn test_palatalized_vowel() {
        assert_eq!(
            Ipa::new("æʲ"),
            Err(Error::PalatalizedVowel('æ'))
        );
        
    }
}
