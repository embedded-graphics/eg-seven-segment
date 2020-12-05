use bitflags::bitflags;
use core::convert::TryFrom;

// Segment layout:
//  AAAAA
// F     B
// F     B
// F     B
//  GGGGG
// E     C
// E     C
// E     C
//  DDDDD

bitflags! {
    pub struct Segments: u8 {
        const A = 0b01000000;
        const B = 0b00100000;
        const C = 0b00010000;
        const D = 0b00001000;
        const E = 0b00000100;
        const F = 0b00000010;
        const G = 0b00000001;
    }
}

impl TryFrom<char> for Segments {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '0' => Self::A | Self::B | Self::C | Self::D | Self::E | Self::F,
            '1' => Self::B | Self::C,
            '2' => Self::A | Self::B | Self::D | Self::E | Self::G,
            '3' => Self::A | Self::B | Self::C | Self::D | Self::G,
            '4' => Self::B | Self::C | Self::F | Self::G,
            '5' => Self::A | Self::C | Self::D | Self::F | Self::G,
            '6' => Self::A | Self::C | Self::D | Self::E | Self::F | Self::G,
            '7' => Self::A | Self::B | Self::C,
            '8' => Self::A | Self::B | Self::C | Self::D | Self::E | Self::F | Self::G,
            '9' => Self::A | Self::B | Self::C | Self::D | Self::F | Self::G,
            // TODO: add hex digits
            _ => return Err(()),
        })
    }
}
