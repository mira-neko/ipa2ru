use std::fmt;
use crate::ipa;

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Vowels {
    A,  E,  I,  O,  U
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Consonants {
    P,  B,  F,  V,  K,  G,  T,
    D,  Sx, Zx, S,  Z,  J,  L,
    M,  N,  R,  H,  C,  Cq, Pr
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phoneme {
    Vowel { phoneme: Vowels },
    Consonant { phoneme: Consonants, is_palatalized: bool }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PhonemeError {
    NotPalatalizedJ,
    NotPalatalizedCq
}

impl fmt::Debug for PhonemeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PhonemeError::NotPalatalizedJ => {
                write!(formatter, "j must be palatalized")?;
            },
            PhonemeError::NotPalatalizedCq => {
                write!(formatter, "cq must be palatalized")?;
            }
        }
        Ok(())
    }
}

impl Phoneme {
    fn new(phoneme: Phoneme) -> Result<Self, PhonemeError> {
        match phoneme {
            Phoneme::Vowel { phoneme } => Ok(Phoneme::Vowel { phoneme }),
            Phoneme::Consonant { phoneme, is_palatalized } => match phoneme {
                Consonants::J => if is_palatalized { Ok(Phoneme::Consonant { phoneme, is_palatalized }) }
                    else { Err(PhonemeError::NotPalatalizedJ) },
                Consonants::Cq => if is_palatalized { Ok(Phoneme::Consonant { phoneme, is_palatalized }) }
                    else { Err(PhonemeError::NotPalatalizedCq) },
                _ => Ok(Phoneme::Consonant { phoneme, is_palatalized })
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhonemeSeq(Vec<Phoneme>);

impl PhonemeSeq {
    fn new(ipa: ipa::Ipa) -> Self {
        (&ipa).into_iter().fold(Self::default(), Self::next)
    }

    fn next(self, sound: &ipa::Sound) -> Self {
        let mut vec = self.0.clone();
        match *sound {
            ipa::Sound::Vowel { phoneme, is_long } => match phoneme {
                ipa::Vowels::NearOpenFrontUroundedVowel => if is_long {
                    vec.push(Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A }).unwrap());
                    vec.push(Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A }).unwrap());
                } else {
                    vec.push(Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A }).unwrap());
                }
            },
            ipa::Sound::Consonant { phoneme, is_long, is_palatalized } => match phoneme {
                ipa::Consonants::VoicedAlveolarNasal => if is_long {
                    vec.push(Phoneme::new(Phoneme::Consonant { phoneme: Consonants::N, is_palatalized }).unwrap());
                    vec.push(Phoneme::new(Phoneme::Consonant { phoneme: Consonants::N, is_palatalized }).unwrap());
                } else {
                    vec.push(Phoneme::new(Phoneme::Consonant { phoneme: Consonants::N, is_palatalized }).unwrap());
                }
            }
        }
        PhonemeSeq(vec)
    }
}

impl Default for PhonemeSeq {
    fn default() -> Self {
        PhonemeSeq(vec![])
    }
}

impl fmt::Display for PhonemeSeq {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            let is_prev_palatalized = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant {phoneme: _, is_palatalized } => is_palatalized
                }
            };
            let is_vowel_next = if i == self.0.len() - 1 {
                false
            } else {
                match self.0[i + 1] {
                    Phoneme::Vowel { phoneme: _ } => true,
                    Phoneme::Consonant {phoneme: _, is_palatalized: _ } => false
                }
            };
            let is_consonant_prev = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant {phoneme: _, is_palatalized: _ } => true
                }
            };
            let is_cq_or_sq_prev = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant {phoneme, is_palatalized } => match phoneme {
                        Consonants::Sx => if is_palatalized {
                            true
                        } else {
                            false
                        },
                        Consonants::Cq => true,
                        _ => false
                    }
                }
            };
            match self.0[i] {
                Phoneme::Vowel { phoneme } => {
                    if is_prev_palatalized && !is_cq_or_sq_prev {
                        match phoneme {
                            Vowels::A => { write!(formatter, "я")?; },
                            Vowels::E => { write!(formatter, "е")?; },
                            Vowels::I => { write!(formatter, "и")?; },
                            Vowels::O => { write!(formatter, "ё")?; },
                            Vowels::U => { write!(formatter, "ю")?; },
                        }
                    } else {
                        match phoneme {
                            Vowels::A => { write!(formatter, "а")?; },
                            Vowels::E => { write!(formatter, "э")?; },
                            Vowels::I => { write!(formatter, "ы")?; },
                            Vowels::O => { write!(formatter, "о")?; },
                            Vowels::U => { write!(formatter, "у")?; },
                        }
                    }
                },
                Phoneme::Consonant {phoneme, is_palatalized } => {
                    match phoneme {
                        Consonants::P  => { write!(formatter, "п")?; },
                        Consonants::B  => { write!(formatter, "б")?; },
                        Consonants::F  => { write!(formatter, "ф")?; },
                        Consonants::V  => { write!(formatter, "в")?; },
                        Consonants::K  => { write!(formatter, "к")?; },
                        Consonants::G  => { write!(formatter, "г")?; },
                        Consonants::T  => { write!(formatter, "т")?; },
                        Consonants::D  => { write!(formatter, "д")?; },
                        Consonants::Sx => if is_palatalized {
                            write!(formatter, "щ")?;
                        } else {
                            write!(formatter, "ш")?;
                        },
                        Consonants::Zx => { write!(formatter, "ж")?; },
                        Consonants::S  => { write!(formatter, "с")?; },
                        Consonants::Z  => { write!(formatter, "з")?; },
                        Consonants::J  => if is_vowel_next && is_consonant_prev {
                                write!(formatter, "ъ")?;
                            } else if !is_vowel_next {
                                write!(formatter, "й")?;
                            },
                        Consonants::L  => { write!(formatter, "л")?; },
                        Consonants::M  => { write!(formatter, "м")?; },
                        Consonants::N  => { write!(formatter, "н")?; },
                        Consonants::R  => { write!(formatter, "р")?; },
                        Consonants::H  => { write!(formatter, "х")?; },
                        Consonants::C  => { write!(formatter, "с")?; },
                        Consonants::Cq => { write!(formatter, "ч")?; },
                        Consonants::Pr => { write!(formatter, " ")?; },
                    }
                    if is_palatalized && phoneme != Consonants::J && !is_vowel_next {
                        write!(formatter, "ь")?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct Ru(PhonemeSeq);

impl Ru {
    pub fn new(ipa: ipa::Ipa) -> Self {
        Ru(PhonemeSeq::new(ipa))
    }
}

impl fmt::Display for Ru {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[cfg(test)]
mod ru_phoneme_build_tests {
    use super::*;

    #[test]
    fn test_ok() {
        assert_eq!(
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A }),
            Ok(Phoneme::Vowel { phoneme: Vowels::A })
        );
        assert_eq!(
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::B, is_palatalized: false }),
            Ok(Phoneme::Consonant { phoneme: Consonants::B, is_palatalized: false })
        );
        assert_eq!(
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::J, is_palatalized: true }),
            Ok(Phoneme::Consonant { phoneme: Consonants::J, is_palatalized: true })
        );
        assert_eq!(
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::N, is_palatalized: true }),
            Ok(Phoneme::Consonant { phoneme: Consonants::N, is_palatalized: true })
        );
    }

    #[test]
    fn test_not_palatalized_j() {
        assert_eq!(
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::J, is_palatalized: false }),
            Err(PhonemeError::NotPalatalizedJ)
        );
    }

    #[test]
    fn test_not_palatalized_cq() {
        assert_eq!(
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::Cq, is_palatalized: false }),
            Err(PhonemeError::NotPalatalizedCq)
        );
    }
}

#[cfg(test)]
mod ru_phoneme_seq_fmt_tests {
    use super::*;

    #[test]
    fn test_nya() -> Result<(), PhonemeError> {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::N, is_palatalized: true })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A })?,
        ])), "ня");
        Ok(())
    }

    #[test]
    fn test_jer() -> Result<(), PhonemeError> {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::P, is_palatalized: false })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::O })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::D, is_palatalized: false })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::J, is_palatalized: true })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::E })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::Z, is_palatalized: false })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::D, is_palatalized: false })?,
        ])), "подъезд");
        Ok(())
    }

    #[test]
    fn test_huj() -> Result<(), PhonemeError> {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::H, is_palatalized: false })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::U })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::J, is_palatalized: true })?,
        ])), "хуй");
        Ok(())
    }

    #[test]
    fn test_intervokalnyj_jot() -> Result<(), PhonemeError> {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::H, is_palatalized: false })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::U })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::J, is_palatalized: true })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::E })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::T, is_palatalized: true })?,
        ])), "ахуеть");
        Ok(())
    }

    #[test]
    fn test_nacqalnyj_jot() -> Result<(), PhonemeError> {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::J, is_palatalized: true })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::E })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::B, is_palatalized: false })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::T, is_palatalized: true })?,
        ])), "ебать");
        Ok(())
    }

    #[test]
    fn test_squsxa() -> Result<(), PhonemeError> {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::Sx, is_palatalized: true })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::U })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::Sx, is_palatalized: false })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A })?,
        ])), "щуша");
        Ok(())
    }

    #[test]
    fn test_cqakra() -> Result<(), PhonemeError> {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::Cq, is_palatalized: true })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::K, is_palatalized: false })?,
            Phoneme::new(Phoneme::Consonant { phoneme: Consonants::R, is_palatalized: false })?,
            Phoneme::new(Phoneme::Vowel { phoneme: Vowels::A })?,
        ])), "чакра");
        Ok(())
    }
}

#[cfg(test)]
mod ru_integration_tests {
    use super::*;

    #[test]
    fn test_nya() -> Result<(), PhonemeError> {
        assert_eq!(
            format!("{}", Ru::new(ipa::Ipa::new("nʲæ").unwrap())),
            "ня"
        );
        Ok(())
    }
}
