use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
enum Vowels {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
enum Consonants {}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd)]
enum Sound {
    Vowel { phoneme: Vowels, is_long: bool },
    Consonant { phoneme: Consonants, is_long: bool, is_palatalized: bool }
}

#[allow(dead_code)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum SoundError {
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
    #[allow(dead_code)]
    fn new(_phoneme: Sound) -> Result<Self, SoundError> {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RuVowels {
    A,  E,  I,  O,  U
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RuConsonants {
    P,  B,  F,  V,  K,  G,  T,  D, Sx, Zx,
    S,  Z,  J,  L,  M,  N,  R,  H, C,  Cq, Pr
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RuPhoneme {
    Vowel { phoneme: RuVowels },
    Consonant { phoneme: RuConsonants, is_palatalized: bool }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum RuPhonemeError {
    NotPalatalizedJ,
    NotPalatalizedCq
}

impl fmt::Debug for RuPhonemeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuPhonemeError::NotPalatalizedJ => {
                write!(formatter, "j must be palatalized")?;
            },
            RuPhonemeError::NotPalatalizedCq => {
                write!(formatter, "cq must be palatalized")?;
            }
        }
        Ok(())
    }
}

impl RuPhoneme {
    #[allow(dead_code)]
    fn new(phoneme: RuPhoneme) -> Result<Self, RuPhonemeError> {
        match phoneme {
            RuPhoneme::Vowel { phoneme } => Ok(RuPhoneme::Vowel { phoneme }),
            RuPhoneme::Consonant { phoneme, is_palatalized } => match phoneme {
                RuConsonants::J => if is_palatalized { Ok(RuPhoneme::Consonant { phoneme, is_palatalized }) }
                    else { Err(RuPhonemeError::NotPalatalizedJ) },
                RuConsonants::Cq => if is_palatalized { Ok(RuPhoneme::Consonant { phoneme, is_palatalized }) }
                    else { Err(RuPhonemeError::NotPalatalizedCq) },
                _ => Ok(RuPhoneme::Consonant { phoneme, is_palatalized })
            }
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
                _ => match self.0[i - 1] {
                    RuPhoneme::Vowel { phoneme: _ } => false,
                    RuPhoneme::Consonant {phoneme: _, is_palatalized } => is_palatalized
                }
            };
            let is_vowel_next = if i == self.0.len() - 1 {
                false
            } else {
                match self.0[i + 1] {
                    RuPhoneme::Vowel { phoneme: _ } => true,
                    RuPhoneme::Consonant {phoneme: _, is_palatalized: _ } => false
                }
            };
            let is_consonant_prev = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    RuPhoneme::Vowel { phoneme: _ } => false,
                    RuPhoneme::Consonant {phoneme: _, is_palatalized: _ } => true
                }
            };
            let is_cq_or_sq_prev = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    RuPhoneme::Vowel { phoneme: _ } => false,
                    RuPhoneme::Consonant {phoneme, is_palatalized } => match phoneme {
                        RuConsonants::Sx => if is_palatalized {
                            true
                        } else {
                            false
                        },
                        RuConsonants::Cq => true,
                        _ => false
                    }
                }
            };
            match self.0[i] {
                RuPhoneme::Vowel { phoneme } => {
                    if is_prev_palatalized && !is_cq_or_sq_prev {
                        match phoneme {
                            RuVowels::A => { write!(formatter, "я")?; },
                            RuVowels::E => { write!(formatter, "е")?; },
                            RuVowels::I => { write!(formatter, "и")?; },
                            RuVowels::O => { write!(formatter, "ё")?; },
                            RuVowels::U => { write!(formatter, "ю")?; },
                        }
                    } else {
                        match phoneme {
                            RuVowels::A => { write!(formatter, "а")?; },
                            RuVowels::E => { write!(formatter, "э")?; },
                            RuVowels::I => { write!(formatter, "ы")?; },
                            RuVowels::O => { write!(formatter, "о")?; },
                            RuVowels::U => { write!(formatter, "у")?; },
                        }
                    }
                },
                RuPhoneme::Consonant {phoneme, is_palatalized } => {
                    match phoneme {
                        RuConsonants::P  => { write!(formatter, "п")?; },
                        RuConsonants::B  => { write!(formatter, "б")?; },
                        RuConsonants::F  => { write!(formatter, "ф")?; },
                        RuConsonants::V  => { write!(formatter, "в")?; },
                        RuConsonants::K  => { write!(formatter, "к")?; },
                        RuConsonants::G  => { write!(formatter, "г")?; },
                        RuConsonants::T  => { write!(formatter, "т")?; },
                        RuConsonants::D  => { write!(formatter, "д")?; },
                        RuConsonants::Sx => if is_palatalized {
                            write!(formatter, "щ")?;
                        } else {
                            write!(formatter, "ш")?;
                        },
                        RuConsonants::Zx => { write!(formatter, "ж")?; },
                        RuConsonants::S  => { write!(formatter, "с")?; },
                        RuConsonants::Z  => { write!(formatter, "з")?; },
                        RuConsonants::J  => if is_vowel_next && is_consonant_prev {
                                write!(formatter, "ъ")?;
                            } else if !is_vowel_next {
                                write!(formatter, "й")?;
                            },
                        RuConsonants::L  => { write!(formatter, "л")?; },
                        RuConsonants::M  => { write!(formatter, "м")?; },
                        RuConsonants::N  => { write!(formatter, "н")?; },
                        RuConsonants::R  => { write!(formatter, "р")?; },
                        RuConsonants::H  => { write!(formatter, "х")?; },
                        RuConsonants::C  => { write!(formatter, "с")?; },
                        RuConsonants::Cq => { write!(formatter, "ч")?; },
                        RuConsonants::Pr => { write!(formatter, " ")?; },
                    }
                    if is_palatalized && phoneme != RuConsonants::J && !is_vowel_next {
                        write!(formatter, "ь")?;
                    }
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
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::A }),
            Ok(RuPhoneme::Vowel { phoneme: RuVowels::A })
        );
        assert_eq!(
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::B, is_palatalized: false }),
            Ok(RuPhoneme::Consonant { phoneme: RuConsonants::B, is_palatalized: false })
        );
        assert_eq!(
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::J, is_palatalized: true }),
            Ok(RuPhoneme::Consonant { phoneme: RuConsonants::J, is_palatalized: true })
        );
        assert_eq!(
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::N, is_palatalized: true }),
            Ok(RuPhoneme::Consonant { phoneme: RuConsonants::N, is_palatalized: true })
        );
    }

    #[test]
    fn test_not_palatalized_j() {
        assert_eq!(
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::J, is_palatalized: false }),
            Err(RuPhonemeError::NotPalatalizedJ)
        );
    }

    #[test]
    fn test_not_palatalized_cq() {
        assert_eq!(
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::Cq, is_palatalized: false }),
            Err(RuPhonemeError::NotPalatalizedCq)
        );
    }
}

#[cfg(test)]
mod ru_phoneme_seq_fmt_tests {
    use super::*;

    #[test]
    fn test_nya() -> Result<(), RuPhonemeError> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::N, is_palatalized: true })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::A })?,
        ])), "ня");
        Ok(())
    }

    #[test]
    fn test_jer() -> Result<(), RuPhonemeError> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::P, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::O })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::D, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::J, is_palatalized: true })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::E })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::Z, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::D, is_palatalized: false })?,
        ])), "подъезд");
        Ok(())
    }

    #[test]
    fn test_huj() -> Result<(), RuPhonemeError> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::H, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::U })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::J, is_palatalized: true })?,
        ])), "хуй");
        Ok(())
    }

    #[test]
    fn test_intervokalnyj_jot() -> Result<(), RuPhonemeError> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::A })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::H, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::U })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::J, is_palatalized: true })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::E })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::T, is_palatalized: true })?,
        ])), "ахуеть");
        Ok(())
    }

    #[test]
    fn test_nacqalnyj_jot() -> Result<(), RuPhonemeError> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::J, is_palatalized: true })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::E })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::B, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::A })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::T, is_palatalized: true })?,
        ])), "ебать");
        Ok(())
    }

    #[test]
    fn test_squsxa() -> Result<(), RuPhonemeError> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::Sx, is_palatalized: true })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::U })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::Sx, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::A })?,
        ])), "щуша");
        Ok(())
    }

    #[test]
    fn test_cqakra() -> Result<(), RuPhonemeError> {
        assert_eq!(format!("{}", RuPhonemeSeq(vec![
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::Cq, is_palatalized: true })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::A })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::K, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Consonant { phoneme: RuConsonants::R, is_palatalized: false })?,
            RuPhoneme::new(RuPhoneme::Vowel { phoneme: RuVowels::A })?,
        ])), "чакра");
        Ok(())
    }
}
