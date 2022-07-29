use std::fmt;

enum Sounds {}

enum RuPhonemes {
    A,  E,  I,  O,  U,  Y,  P,
    B,  F,  V,  K,  G,  T,  D,
    Sx, Sq, Zx, S,  Z,  J,  L,
    M,  N,  R,  H,  C,  Cq
}

struct Sound {
    sound: Sounds,
    is_palatalized: bool,
    is_long: bool,
}

struct RuPhoneme {
    phoneme: RuPhonemes,
    is_palatalized: bool,
}

pub struct IPA(Vec<Sound>);

impl IPA {
    pub fn new(ipa: String) -> Self {
        todo!()
    }

    fn get(self) -> std::vec::IntoIter<Sound> {
        self.0.into_iter()
    }
}

pub struct RuPhonemeSec(Vec<RuPhoneme>);

impl RuPhonemeSec {
    pub fn new(ipa: IPA) -> Self {
        ipa.get().fold(Self::default(), Self::next)
    }

    fn next(self, sound: Sound) -> Self {
        todo!()
    }
}

impl Default for RuPhonemeSec {
    fn default() -> Self {
        todo!()
    }
}

impl fmt::Display for RuPhonemeSec {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
