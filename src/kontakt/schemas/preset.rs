use super::{Kon4, XMLDocument};

#[derive(Debug)]
pub enum KontaktPreset {
    Kon1(XMLDocument),
    Kon2(XMLDocument),
    Kon3(XMLDocument),
    Kon4(Kon4),
}

impl KontaktPreset {}
