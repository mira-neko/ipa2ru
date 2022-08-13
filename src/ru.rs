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
    P, B, F, V, K, G,
    T, D, W, X, S, Z,
    L, M, N, R, H, C
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PalatalizedOnlyConsonants {
    J, Q
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phoneme {
    Vowel { phoneme: Vowels },
    Consonant { phoneme: Consonants, is_palatalized: bool },
    PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants },
    Probel
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PhonemeSeq(Vec<Phoneme>);

macro_rules! push_phoneme {
    ($vec:ident, $is_long:ident, $phoneme:expr) => {
        if $is_long {
            $vec.push($phoneme);
            $vec.push($phoneme);
        } else {
            $vec.push($phoneme);
        }
    };
}

macro_rules! push_vowel {
    ($vec:ident, $is_long:ident, $phoneme:ident) => {
        push_phoneme!($vec, $is_long, Phoneme::Vowel { phoneme: Vowels::$phoneme })
    };
}

macro_rules! push_consonant {
    ($vec:ident, $is_long:ident, $is_palatalized:ident, $phoneme:ident) => {
        push_phoneme!($vec, $is_long, Phoneme::Consonant { phoneme: Consonants::$phoneme, $is_palatalized })
    };
}

impl PhonemeSeq {
    fn new(ipa: ipa::Ipa) -> Self {
        (&ipa).into_iter().fold(Self::default(), Self::next)
    }

    fn next(self, sound: &ipa::Sound) -> Self {
        let mut vec = self.0.clone();
        match *sound {
            ipa::Sound::Vowel { phoneme, is_long } => match phoneme {
                ipa::Vowels::CloseBackRoundedVowel => push_vowel!(vec, is_long, U),
                ipa::Vowels::NearOpenFrontUroundedVowel => push_vowel!(vec, is_long, A),
            },
            ipa::Sound::Consonant { phoneme, is_long, is_palatalized } => match phoneme {
                ipa::Consonants::VoicedAlveolarNasal => push_consonant!(vec, is_long, is_palatalized, N),
                ipa::Consonants::VoicedBilabialNasal => push_consonant!(vec, is_long, is_palatalized, M),
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

#[deny(unused_must_use)]
impl fmt::Display for PhonemeSeq {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.0.len() {
            let is_prev_palatalized = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant { phoneme: _, is_palatalized } => is_palatalized,
                    Phoneme::PalatalizedOnlyConsonant { phoneme: _ } => true,
                    Phoneme::Probel => false
                }
            };
            let is_vowel_next = if i == self.0.len() - 1 {
                false
            } else {
                match self.0[i + 1] {
                    Phoneme::Vowel { phoneme: _ } => true,
                    Phoneme::Consonant { phoneme: _, is_palatalized: _ } => false,
                    Phoneme::PalatalizedOnlyConsonant { phoneme: _ } => false,
                    Phoneme::Probel => false
                }
            };
            let is_consonant_prev = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant { phoneme: _, is_palatalized: _ } => true,
                    Phoneme::PalatalizedOnlyConsonant { phoneme: _ } => true,
                    Phoneme::Probel => false
                }
            };
            let is_q_or_wj_prev = match i {
                0 => false,
                _ => match self.0[i - 1] {
                    Phoneme::Vowel { phoneme: _ } => false,
                    Phoneme::Consonant { phoneme, is_palatalized } => match phoneme {
                        Consonants::W => if is_palatalized {
                            true
                        } else {
                            false
                        },
                        _ => false
                    },
                    Phoneme::PalatalizedOnlyConsonant { phoneme } => match phoneme {
                        PalatalizedOnlyConsonants::Q => true,
                        _ => false
                    },
                    Phoneme::Probel => false
                }
            };
            match self.0[i] {
                Phoneme::Vowel { phoneme } => {
                    if is_prev_palatalized && !is_q_or_wj_prev {
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
                        Consonants::P => { write!(formatter, "п")?; },
                        Consonants::B => { write!(formatter, "б")?; },
                        Consonants::F => { write!(formatter, "ф")?; },
                        Consonants::V => { write!(formatter, "в")?; },
                        Consonants::K => { write!(formatter, "к")?; },
                        Consonants::G => { write!(formatter, "г")?; },
                        Consonants::T => { write!(formatter, "т")?; },
                        Consonants::D => { write!(formatter, "д")?; },
                        Consonants::W => if is_palatalized {
                            write!(formatter, "щ")?;
                        } else {
                            write!(formatter, "ш")?;
                        },
                        Consonants::X => { write!(formatter, "ж")?; },
                        Consonants::S => { write!(formatter, "с")?; },
                        Consonants::Z => { write!(formatter, "з")?; },
                        Consonants::L => { write!(formatter, "л")?; },
                        Consonants::M => { write!(formatter, "м")?; },
                        Consonants::N => { write!(formatter, "н")?; },
                        Consonants::R => { write!(formatter, "р")?; },
                        Consonants::H => { write!(formatter, "х")?; },
                        Consonants::C => { write!(formatter, "с")?; },
                    }
                    if is_palatalized && !is_vowel_next {
                        write!(formatter, "ь")?;
                    }
                },
                Phoneme::PalatalizedOnlyConsonant { phoneme } => match phoneme {
                    PalatalizedOnlyConsonants::J => if is_vowel_next && is_consonant_prev {
                        write!(formatter, "ъ")?;
                    } else if !is_vowel_next {
                        write!(formatter, "й")?;
                    },
                    PalatalizedOnlyConsonants::Q => { write!(formatter, "ч")?; }
                },
                Phoneme::Probel => { write!(formatter, " ")?; }
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
mod ru_phoneme_seq_fmt_tests {
    use super::*;

    #[test]
    fn test_nja() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::N, is_palatalized: true },
            Phoneme::Vowel { phoneme: Vowels::A },
        ])), "ня");
    }

    #[test]
    fn test_jer() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::P, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::O },
            Phoneme::Consonant { phoneme: Consonants::D, is_palatalized: false },
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
            Phoneme::Vowel { phoneme: Vowels::E },
            Phoneme::Consonant { phoneme: Consonants::Z, is_palatalized: false },
            Phoneme::Consonant { phoneme: Consonants::D, is_palatalized: false },
        ])), "подъезд");
    }

    #[test]
    fn test_huj() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::H, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::U },
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
        ])), "хуй");
    }

    #[test]
    fn test_intervokalnyj_jot() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Vowel { phoneme: Vowels::A },
            Phoneme::Consonant { phoneme: Consonants::H, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::U },
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
            Phoneme::Vowel { phoneme: Vowels::E },
            Phoneme::Consonant { phoneme: Consonants::T, is_palatalized: true },
        ])), "ахуеть");
    }

    #[test]
    fn test_nacqalnyj_jot() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
            Phoneme::Vowel { phoneme: Vowels::E },
            Phoneme::Consonant { phoneme: Consonants::B, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::A },
            Phoneme::Consonant { phoneme: Consonants::T, is_palatalized: true },
        ])), "ебать");
    }

    #[test]
    fn test_squsxa() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::W, is_palatalized: true },
            Phoneme::Vowel { phoneme: Vowels::U },
            Phoneme::Consonant { phoneme: Consonants::W, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::A },
        ])), "щуша");
    }

    #[test]
    fn test_cqakra() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::Q },
            Phoneme::Vowel { phoneme: Vowels::A },
            Phoneme::Consonant { phoneme: Consonants::K, is_palatalized: false },
            Phoneme::Consonant { phoneme: Consonants::R, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::A },
        ])), "чакра");
    }
}

#[cfg(test)]
mod ru_integration_tests {
    use super::*;

    #[test]
    fn test_nja() {
        assert_eq!(
            format!("{}", Ru::new(ipa::Ipa::new("nʲæ").unwrap())),
            "ня"
        );
    }

    #[test]
    fn test_mjaau() {
        assert_eq!(
            format!("{}", Ru::new(ipa::Ipa::new("mʲæːu").unwrap())),
            "мяау"
        );
    }

    #[test]
    fn test_mmjaau() {
        assert_eq!(
            format!("{}", Ru::new(ipa::Ipa::new("mʲːæːu").unwrap())),
            "мьмяау"
        );
    }
}
