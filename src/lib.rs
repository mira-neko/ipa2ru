use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Sounds {}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RuPhonemes {
    A,  E,  I,  O,  U,  Y,  P,
    B,  F,  V,  K,  G,  T,  D,
    Sx, Sq, Zx, S,  Z,  J,  L,
    M,  N,  R,  H,  C,  Cq
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
pub struct RuPhonemeSec(Vec<RuPhoneme>);

impl RuPhonemeSec {
    pub fn new(ipa: Ipa) -> Self {
        ipa.get_as_vec().into_iter().fold(Self::default(), Self::next)
    }

    fn next(self, _sound: Sound) -> Self {
        todo!()
    }
}

impl Default for RuPhonemeSec {
    fn default() -> Self {
        todo!()
    }
}

impl fmt::Display for RuPhonemeSec {
    fn fmt(&self, _formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
