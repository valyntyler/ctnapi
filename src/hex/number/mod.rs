use crate::Weight;

pub enum HexNumber {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    _10,
    _11,
    _12,
}

impl Weight for HexNumber {
    fn weight(&self) -> usize {
        match self {
            HexNumber::_2 | HexNumber::_12 => 1,
            _ => 2,
        }
    }
}
