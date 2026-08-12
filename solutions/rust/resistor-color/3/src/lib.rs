use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use std::fmt::Display;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use ResistorColor::*;
        f.write_str(match self {
            Black => "Black",
            Brown => "Brown",
            Red => "Red",
            Orange => "Orange",
            Yellow => "Yellow",
            Green => "Green",
            Blue => "Blue",
            Violet => "Violet",
            Grey => "Grey",
            White => "White",
        })
    }
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::from_int(value as u8) {
        Ok(resistor) => resistor.to_string(),
        Err(_) => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    ResistorColor::into_enum_iter().collect()
}
