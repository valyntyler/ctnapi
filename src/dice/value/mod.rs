use crate::Weight;

pub enum DiceValue {
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

impl Weight for DiceValue {
    fn weight(&self) -> usize {
        match self {
            DiceValue::_2 | DiceValue::_12 => 1,
            DiceValue::_3 | DiceValue::_11 => 2,
            DiceValue::_4 | DiceValue::_10 => 3,
            DiceValue::_5 | DiceValue::_9 => 4,
            DiceValue::_6 | DiceValue::_8 => 5,
            DiceValue::_7 => 6,
        }
    }
}
