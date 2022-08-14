use std::fmt;
use crate::ipa;

fn either<T>(which: bool, if_false: T, if_true: T) -> T {
    if which {
        if_true
    } else {
        if_false
    }
}

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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
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

macro_rules! push_palatalized_only_consonant {
    ($vec:ident, $is_long:ident, $phoneme:ident) => {
        push_phoneme!($vec, $is_long, Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::$phoneme })
    };
}

impl PhonemeSeq {
    fn new(ipa: ipa::Ipa) -> Self {
        (&ipa).iter().fold(Self::default(), Self::next)
    }

    fn next(self, sound: &ipa::Sound) -> Self {
        let mut vec = self.0;
        match *sound {
            ipa::Sound::Vowel { phoneme, is_long } => match phoneme {
                ipa::Vowels::CloseBackRounded => push_vowel!(vec, is_long, U),
                ipa::Vowels::MidCentral => push_vowel!(vec, is_long, A),
                ipa::Vowels::NearOpenFrontUrounded => push_vowel!(vec, is_long, A),
                ipa::Vowels::OpenBackUnrounded => push_vowel!(vec, is_long, A),
                ipa::Vowels::OpenFrontUnrounded => push_vowel!(vec, is_long, A),
                ipa::Vowels::OpenMidBackUnrounded => push_vowel!(vec, is_long, A),
            },
            ipa::Sound::Consonant { phoneme, is_long, is_palatalized } => match phoneme {
                ipa::Consonants::VoicedAlveolarNasal => push_consonant!(vec, is_long, is_palatalized, N),
                ipa::Consonants::VoicedBilabialNasal => push_consonant!(vec, is_long, is_palatalized, M),
                ipa::Consonants::VoicedPalatalApproximant => push_palatalized_only_consonant!(vec, is_long, J),
                ipa::Consonants::VoicelessBilabialPlosive => push_consonant!(vec, is_long, is_palatalized, P),
            }
        }
        PhonemeSeq(vec)
    }
}

macro_rules! write_letter {
    ($formatter:ident, $letter:expr) => {
        {
            write!($formatter, "{}", $letter)?;
        }
    };
}

macro_rules! write_either_letter {
    ($formatter:ident, $which:ident, $letter_false:expr, $letter_true:expr) => {
        write_letter!($formatter, either($which, $letter_false, $letter_true))
    };
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
                        Consonants::W => is_palatalized,
                        _ => false
                    },
                    Phoneme::PalatalizedOnlyConsonant { phoneme } => matches!(phoneme, PalatalizedOnlyConsonants::Q),
                    Phoneme::Probel => false
                }
            };
            match self.0[i] {
                Phoneme::Vowel { phoneme } => {
                    let is_vowel_palatalizing = is_prev_palatalized && !is_q_or_wj_prev;
                    match phoneme {
                        Vowels::A => write_either_letter!(formatter, is_vowel_palatalizing, "а", "я"),
                        Vowels::E => write_either_letter!(formatter, is_vowel_palatalizing, "э", "е"),
                        Vowels::I => write_either_letter!(formatter, is_vowel_palatalizing, "ы", "и"),
                        Vowels::O => write_either_letter!(formatter, is_vowel_palatalizing, "о", "ё"),
                        Vowels::U => write_either_letter!(formatter, is_vowel_palatalizing, "у", "ю"),
                    }
                },
                Phoneme::Consonant {phoneme, is_palatalized } => {
                    match phoneme {
                        Consonants::P => write_letter!(formatter, "п"),
                        Consonants::B => write_letter!(formatter, "б"),
                        Consonants::F => write_letter!(formatter, "ф"),
                        Consonants::V => write_letter!(formatter, "в"),
                        Consonants::K => write_letter!(formatter, "к"),
                        Consonants::G => write_letter!(formatter, "г"),
                        Consonants::T => write_letter!(formatter, "т"),
                        Consonants::D => write_letter!(formatter, "д"),
                        Consonants::W => write_either_letter!(formatter, is_palatalized, "ш", "щ"),
                        Consonants::X => write_letter!(formatter, "ж"),
                        Consonants::S => write_letter!(formatter, "с"),
                        Consonants::Z => write_letter!(formatter, "з"),
                        Consonants::L => write_letter!(formatter, "л"),
                        Consonants::M => write_letter!(formatter, "м"),
                        Consonants::N => write_letter!(formatter, "н"),
                        Consonants::R => write_letter!(formatter, "р"),
                        Consonants::H => write_letter!(formatter, "х"),
                        Consonants::C => write_letter!(formatter, "с"),
                    }
                    if is_palatalized && !is_vowel_next {
                        write_letter!(formatter, "ь");
                    }
                },
                Phoneme::PalatalizedOnlyConsonant { phoneme } => match phoneme {
                    PalatalizedOnlyConsonants::J => if is_vowel_next && is_consonant_prev {
                        write_letter!(formatter, "ъ");
                    } else if !is_vowel_next {
                        write_letter!(formatter, "й");
                    },
                    PalatalizedOnlyConsonants::Q => write_letter!(formatter, "ч")
                },
                Phoneme::Probel => write_letter!(formatter, " ")
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
        self.0.fmt(formatter)
    }
}

#[cfg(test)]
mod ru_phoneme_seq_fmt_tests {
    use super::*;

    #[test]
    fn test_na() {
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
    fn test_intervokalnij_jot() {
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
    fn test_naqalnij_jot() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::PalatalizedOnlyConsonant { phoneme: PalatalizedOnlyConsonants::J },
            Phoneme::Vowel { phoneme: Vowels::E },
            Phoneme::Consonant { phoneme: Consonants::B, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::A },
            Phoneme::Consonant { phoneme: Consonants::T, is_palatalized: true },
        ])), "ебать");
    }

    #[test]
    fn test_wuwa() {
        assert_eq!(format!("{}", PhonemeSeq(vec![
            Phoneme::Consonant { phoneme: Consonants::W, is_palatalized: true },
            Phoneme::Vowel { phoneme: Vowels::U },
            Phoneme::Consonant { phoneme: Consonants::W, is_palatalized: false },
            Phoneme::Vowel { phoneme: Vowels::A },
        ])), "щуша");
    }

    #[test]
    fn test_qakra() {
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
    fn test_na() {
        assert_eq!(
            format!("{}", Ru::new(ipa::Ipa::new("nʲæ").unwrap())),
            "ня"
        );
    }

    #[test]
    fn test_maau() {
        assert_eq!(
            format!("{}", Ru::new(ipa::Ipa::new("mʲæːu").unwrap())),
            "мяау"
        );
    }

    #[test]
    fn test_mmaau() {
        assert_eq!(
            format!("{}", Ru::new(ipa::Ipa::new("mʲːæːu").unwrap())),
            "мьмяау"
        );
    }
}
