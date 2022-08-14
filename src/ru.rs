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

impl PhonemeSeq {
    fn new(ipa: ipa::Ipa) -> Self {
        (&ipa).iter().fold(Self::default(), Self::next)
    }

    fn next(self, sound: &ipa::Sound) -> Self {
        use ipa::{Consonants::*, Vowels::*};
        use PalatalizedOnlyConsonants::*;
        use Consonants::*;
        use Phoneme::*;
        use Vowels::*;

        let mut vec = self.0;
        let next_sound = match *sound {
            ipa::Sound::Vowel { phoneme, is_long } => match phoneme {
                CloseBackRounded      => (Vowel { phoneme: U }, is_long),
                CloseMidFrontRounded  => (Vowel { phoneme: O }, is_long),
                MidCentral            => (Vowel { phoneme: A }, is_long),
                NearOpenFrontUrounded => (Vowel { phoneme: A }, is_long),
                OpenBackUnrounded     => (Vowel { phoneme: A }, is_long),
                OpenFrontUnrounded    => (Vowel { phoneme: A }, is_long),
                OpenMidBackUnrounded  => (Vowel { phoneme: A }, is_long),
            },
            ipa::Sound::Consonant { phoneme, is_long, is_palatalized } => match phoneme {
                VoicedAlveolarNasal      => (Consonant { phoneme: N, is_palatalized }, is_long),
                VoicedBilabialNasal      => (Consonant { phoneme: M, is_palatalized }, is_long),
                VoicedPalatalApproximant => (PalatalizedOnlyConsonant  { phoneme: J }, is_long),
                VoicelessBilabialPlosive => (Consonant { phoneme: P, is_palatalized }, is_long),
            }
        };
        if next_sound.1 {
            vec.push(next_sound.0);
            vec.push(next_sound.0);
        } else {
            vec.push(next_sound.0);
        }
        Self(vec)
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
                        Consonants::W => is_palatalized,
                        _ => false
                    },
                    Phoneme::PalatalizedOnlyConsonant { phoneme } => matches!(phoneme, PalatalizedOnlyConsonants::Q),
                    Phoneme::Probel => false
                }
            };
            write!(formatter, "{}", match self.0[i] {
                Phoneme::Vowel { phoneme } => {
                    let is_vowel_palatalizing = is_prev_palatalized && !is_q_or_wj_prev;
                    match phoneme {
                        Vowels::A => either(is_vowel_palatalizing, "а", "я"),
                        Vowels::E => either(is_vowel_palatalizing, "э", "е"),
                        Vowels::I => either(is_vowel_palatalizing, "ы", "и"),
                        Vowels::O => either(is_vowel_palatalizing, "о", "ё"),
                        Vowels::U => either(is_vowel_palatalizing, "у", "ю"),
                    }
                },
                Phoneme::Consonant {phoneme, is_palatalized } => {
                    let is_jer = is_palatalized && !is_vowel_next;
                    match phoneme {
                        Consonants::P => either(is_jer, "п", "пь"),
                        Consonants::B => either(is_jer, "б", "бь"),
                        Consonants::F => either(is_jer, "ф", "фь"),
                        Consonants::V => either(is_jer, "в", "вь"),
                        Consonants::K => either(is_jer, "к", "кь"),
                        Consonants::G => either(is_jer, "г", "гь"),
                        Consonants::T => either(is_jer, "т", "ть"),
                        Consonants::D => either(is_jer, "д", "дь"),
                        Consonants::W => either(is_palatalized, "ш", "щ" ),
                        Consonants::X => either(is_jer, "ж", "жь"),
                        Consonants::S => either(is_jer, "с", "сь"),
                        Consonants::Z => either(is_jer, "з", "зь"),
                        Consonants::L => either(is_jer, "л", "ль"),
                        Consonants::M => either(is_jer, "м", "мь"),
                        Consonants::N => either(is_jer, "н", "нь"),
                        Consonants::R => either(is_jer, "р", "рь"),
                        Consonants::H => either(is_jer, "х", "хь"),
                        Consonants::C => either(is_jer, "с", "сь"),
                    }
                },
                Phoneme::PalatalizedOnlyConsonant { phoneme } => match phoneme {
                    PalatalizedOnlyConsonants::J => if is_vowel_next && is_consonant_prev {
                        "ъ"
                    } else if !is_vowel_next {
                        "й"
                    } else {
                        ""
                    },
                    PalatalizedOnlyConsonants::Q => "ч"
                },
                Phoneme::Probel => " "
            })?;
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
