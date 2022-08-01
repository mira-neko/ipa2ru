use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
enum Sounds {}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
enum RuPhonemes {
    A,  E,  I,  O,  U,  P,
    B,  F,  V,  K,  G,  T,  D,
    Sx, Sq, Zx, S,  Z,  J,  L,
    M,  N,  R,  H,  C,  Cq, Pr
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VowelOrConsonant {
    Vowel,
    Consonant
}

impl RuPhonemes {
    fn is_vowel_or_consonant(&self) -> Option<VowelOrConsonant> {
        match self {
            v if (v <= &RuPhonemes::U) => Some(VowelOrConsonant::Vowel),
            RuPhonemes::Pr => None,
            _ => Some(VowelOrConsonant::Consonant)
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Sound {
    sound: Sounds,
    is_palatalized: bool,
    is_long: bool,
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RuPhoneme {
    phoneme: RuPhonemes,
    is_palatalized: bool,
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum RuPhonemeErrors {
    PalatalizedVowel,
    NotPalatalizedJ
}

impl fmt::Debug for RuPhonemeErrors {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuPhonemeErrors::PalatalizedVowel => {
                write!(formatter, "vowels cannot be palatalized")?;
            },
            RuPhonemeErrors::NotPalatalizedJ => {
                write!(formatter, "j must be palatalized")?;
            }
        }
        Ok(())
    }
}

impl RuPhoneme {
    #[allow(dead_code)]
    fn new(phoneme: RuPhonemes, is_palatalized: bool) -> Result<Self, RuPhonemeErrors> {
        if phoneme.is_vowel_or_consonant() != Some(VowelOrConsonant::Consonant) {
            if is_palatalized {
                Err(RuPhonemeErrors::PalatalizedVowel)
            } else {
                Ok(RuPhoneme { phoneme, is_palatalized: false })
            }
        } else if phoneme == RuPhonemes::J {
            if is_palatalized {
                Ok(RuPhoneme { phoneme, is_palatalized: true })
            } else {
                Err(RuPhonemeErrors::NotPalatalizedJ)
            }
        } else {
            Ok(RuPhoneme { phoneme, is_palatalized })
        }
    }
}

#[derive(Clone, Debug)]
pub struct Ipa(Vec<Sound>);

impl Ipa {
    pub fn new(_ipa: String) -> Self {
        todo!()
    }

    fn get_as_vec(&self) -> Vec<Sound> {
        self.0.clone()
    }
}

#[derive(Clone, Debug)]
pub struct RuPhonemeSeq(Vec<RuPhoneme>);

impl RuPhonemeSeq {
    pub fn new(ipa: Ipa) -> Self {
        ipa.get_as_vec().into_iter().fold(Self::default(), Self::next)
    }

    fn next(self, _sound: Sound) -> Self {
        todo!()
    }
}

impl Default for RuPhonemeSeq {
    fn default() -> Self {
        todo!()
    }
}

impl fmt::Display for RuPhonemeSeq {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            let is_prev_palatalized = match i {
                0 => false,
                _ => self.0[i - 1].is_palatalized
            };
            let is_vowel_now =
                self.0[i].phoneme.is_vowel_or_consonant()
                == Some(VowelOrConsonant::Vowel);
            let is_vowel_next = if i == self.0.len() - 1 {
                false
            } else {
                self.0[i + 1].phoneme.is_vowel_or_consonant()
                == Some(VowelOrConsonant::Vowel)
            };
            let is_consonant_prev = match i {
                0 => false,
                _ => self.0[i - 1].phoneme.is_vowel_or_consonant()
                    == Some(VowelOrConsonant::Consonant)
            };
            if is_vowel_now {
                if is_prev_palatalized {
                    match self.0[i].phoneme {
                        RuPhonemes::A => { write!(formatter, "я")?; },
                        RuPhonemes::E => { write!(formatter, "е")?; },
                        RuPhonemes::I => { write!(formatter, "и")?; },
                        RuPhonemes::O => { write!(formatter, "ё")?; },
                        RuPhonemes::U => { write!(formatter, "ю")?; },
                        _             => { unreachable!(); },
                    }
                } else {
                    match self.0[i].phoneme {
                        RuPhonemes::A => { write!(formatter, "а")?; },
                        RuPhonemes::E => { write!(formatter, "э")?; },
                        RuPhonemes::I => { write!(formatter, "ы")?; },
                        RuPhonemes::O => { write!(formatter, "о")?; },
                        RuPhonemes::U => { write!(formatter, "у")?; },
                        _             => { unreachable!();          }
                    }
                }
            } else {
                match self.0[i].phoneme {
                    RuPhonemes::P  => { write!(formatter, "п")?; },
                    RuPhonemes::B  => { write!(formatter, "б")?; },
                    RuPhonemes::F  => { write!(formatter, "ф")?; },
                    RuPhonemes::V  => { write!(formatter, "в")?; },
                    RuPhonemes::K  => { write!(formatter, "к")?; },
                    RuPhonemes::G  => { write!(formatter, "г")?; },
                    RuPhonemes::T  => { write!(formatter, "т")?; },
                    RuPhonemes::D  => { write!(formatter, "д")?; },
                    RuPhonemes::Sx => { write!(formatter, "ш")?; },
                    RuPhonemes::Sq => { write!(formatter, "щ")?; },
                    RuPhonemes::Zx => { write!(formatter, "ж")?; },
                    RuPhonemes::S  => { write!(formatter, "с")?; },
                    RuPhonemes::Z  => { write!(formatter, "з")?; },
                    RuPhonemes::J  => if is_vowel_next && is_consonant_prev {
                            write!(formatter, "ъ")?;
                        } else if !is_vowel_next {
                            write!(formatter, "й")?;
                        },
                    RuPhonemes::L  => { write!(formatter, "л")?; },
                    RuPhonemes::M  => { write!(formatter, "м")?; },
                    RuPhonemes::N  => { write!(formatter, "н")?; },
                    RuPhonemes::R  => { write!(formatter, "р")?; },
                    RuPhonemes::H  => { write!(formatter, "х")?; },
                    RuPhonemes::C  => { write!(formatter, "с")?; },
                    RuPhonemes::Cq => { write!(formatter, "ч")?; },
                    RuPhonemes::Pr => { write!(formatter, " ")?; },
                    _              => { unreachable!();          }
                }
                if self.0[i].is_palatalized
                        && self.0[i].phoneme != RuPhonemes::J
                        && !is_vowel_next {
                    write!(formatter, "ь")?;
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod ru_phoneme_build_tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(
            RuPhoneme::new(RuPhonemes::A, false),
            Ok(RuPhoneme { phoneme: RuPhonemes::A, is_palatalized: false })
        );
        assert_eq!(
            RuPhoneme::new(RuPhonemes::B, false),
            Ok(RuPhoneme { phoneme: RuPhonemes::B, is_palatalized: false })
        );
        assert_eq!(
            RuPhoneme::new(RuPhonemes::J, true),
            Ok(RuPhoneme { phoneme: RuPhonemes::J, is_palatalized: true })
        );
        assert_eq!(
            RuPhoneme::new(RuPhonemes::N, true),
            Ok(RuPhoneme { phoneme: RuPhonemes::N, is_palatalized: true })
        );
    }

    #[test]
    fn test_not_palatalized_j() {
        assert_eq!(
            RuPhoneme::new(RuPhonemes::J, false),
            Err(RuPhonemeErrors::NotPalatalizedJ)
        );
    }

    #[test]
    fn test_palatalized_vowel() {
        assert_eq!(
            RuPhoneme::new(RuPhonemes::A, true),
            Err(RuPhonemeErrors::PalatalizedVowel)
        );
    }
}

#[cfg(test)]
mod ru_phoneme_seq_fmt_tests {
    use super::*;

    #[test]
    fn test_nya() -> Result<(), RuPhonemeErrors> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhonemes::N,  true )?,
            RuPhoneme::new(RuPhonemes::A,  false)?,
        ])), "ня");
        Ok(())
    }

    #[test]
    fn test_jer() -> Result<(), RuPhonemeErrors> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhonemes::P,  false)?,
            RuPhoneme::new(RuPhonemes::O,  false)?,
            RuPhoneme::new(RuPhonemes::D,  false)?,
            RuPhoneme::new(RuPhonemes::J,  true )?,
            RuPhoneme::new(RuPhonemes::E,  false)?,
            RuPhoneme::new(RuPhonemes::Z,  false)?,
            RuPhoneme::new(RuPhonemes::D,  false)?,
        ])), "подъезд");
        Ok(())
    }

    #[test]
    fn test_huj() -> Result<(), RuPhonemeErrors> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhonemes::H,  false)?,
            RuPhoneme::new(RuPhonemes::U,  false)?,
            RuPhoneme::new(RuPhonemes::J,  true )?,
        ])), "хуй");
        Ok(())
    }

    #[test]
    fn test_intervokalnyj_jot() -> Result<(), RuPhonemeErrors> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhonemes::A,  false)?,
            RuPhoneme::new(RuPhonemes::H,  false)?,
            RuPhoneme::new(RuPhonemes::U,  false)?,
            RuPhoneme::new(RuPhonemes::J,  true )?,
            RuPhoneme::new(RuPhonemes::E,  false)?,
            RuPhoneme::new(RuPhonemes::T,  true )?,
        ])), "ахуеть");
        Ok(())
    }

    #[test]
    fn test_nacqalnyj_jot() -> Result<(), RuPhonemeErrors> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhonemes::J,  true )?,
            RuPhoneme::new(RuPhonemes::E,  false)?,
            RuPhoneme::new(RuPhonemes::B,  false)?,
            RuPhoneme::new(RuPhonemes::A,  false)?,
            RuPhoneme::new(RuPhonemes::T,  true )?,
        ])), "ебать");
        Ok(())
    }
}
