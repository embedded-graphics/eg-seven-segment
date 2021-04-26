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
    /// Segments.
    pub struct Segments: u8 {
        /// A segment.
        const A = 0b01000000;
        /// B segment.
        const B = 0b00100000;
        /// C segment.
        const C = 0b00010000;
        /// D segment.
        const D = 0b00001000;
        /// E segment.
        const E = 0b00000100;
        /// F segment.
        const F = 0b00000010;
        /// G segment.
        const G = 0b00000001;
    }
}

impl TryFrom<char> for Segments {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            ' ' => Self::empty(),
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
            'a' | 'A' => Self::A | Self::B | Self::C | Self::E | Self::F | Self::G,
            'b' | 'B' => Self::C | Self::D | Self::E | Self::F | Self::G,
            'c' => Self::D | Self::E | Self::G,
            'C' => Self::A | Self::D | Self::E | Self::F,
            'd' | 'D' => Self::B | Self::C | Self::D | Self::E | Self::G,
            'e' | 'E' => Self::A | Self::D | Self::E | Self::F | Self::G,
            'f' | 'F' => Self::A | Self::E | Self::F | Self::G,
            'g' | 'G' => Self::A | Self::C | Self::D | Self::E | Self::F,
            'h' => Self::C | Self::E | Self::F | Self::G,
            'H' => Self::B | Self::C | Self::E | Self::F | Self::G,
            'i' | 'I' => Self::E | Self::F,
            'j' | 'J' => Self::B | Self::C | Self::D | Self::E,
            'l' | 'L' => Self::D | Self::E | Self::F,
            'n' | 'N' => Self::C | Self::E | Self::G,
            'o' => Self::C | Self::D | Self::E | Self::G,
            'O' => Self::A | Self::B | Self::C | Self::D | Self::E | Self::F,
            'p' | 'P' => Self::A | Self::B | Self::E | Self::F | Self::G,
            'q' | 'Q' => Self::A | Self::B | Self::C | Self::F | Self::G,
            'r' | 'R' => Self::E | Self::G,
            's' | 'S' => Self::A | Self::C | Self::D | Self::F | Self::G,
            't' | 'T' => Self::D | Self::E | Self::F | Self::G,
            'u' => Self::C | Self::D | Self::E,
            'U' => Self::B | Self::C | Self::D | Self::E | Self::F,
            'y' | 'Y' => Self::B | Self::C | Self::D | Self::F | Self::G,
            '_' => Self::D,
            '-' => Self::G,
            '=' => Self::D | Self::G,
            '°' => Self::A | Self::B | Self::F | Self::G,
            '"' => Self::B | Self::F,
            '\'' => Self::F,
            '(' | '[' => Self::A | Self::D | Self::E | Self::F,
            ')' | ']' => Self::A | Self::B | Self::C | Self::D,
            '?' => Self::A | Self::B | Self::E | Self::G,
            // TODO: add https://en.wikipedia.org/wiki/Symbols_for_Legacy_Computing ?
            // TODO: document PUA
            '\u{E000}'..='\u{E07F}' => Self::from_bits(value as u8).unwrap(),
            _ => return Err(()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::SevenSegmentStyleBuilder;
    use embedded_graphics::{
        mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::*, text::Text,
    };

    fn test_segments(text: &str, expected_pattern: &[&str]) {
        let style = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(5, 7))
            .digit_spacing(1)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        let mut display = MockDisplay::new();
        Text::new(text, Point::new(0, 6), style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(expected_pattern);
    }

    #[test]
    fn digits() {
        test_segments(
            "0123456789",
            &[
                " ###         ###   ###         ###   ###   ###   ###   ### ",
                "#   #     #     #     # #   # #     #         # #   # #   #",
                "#   #     #     #     # #   # #     #         # #   # #   #",
                "             ###   ###   ###   ###   ###         ###   ### ",
                "#   #     # #         #     #     # #   #     # #   #     #",
                "#   #     # #         #     #     # #   #     # #   #     #",
                " ###         ###   ###         ###   ###         ###   ### ",
            ],
        );
    }

    #[test]
    fn lower_case_letters() {
        test_segments(
            "abcdefghij",
            &[
                " ###                     ###   ###   ###                   ",
                "#   # #               # #     #     #     #     #         #",
                "#   # #               # #     #     #     #     #         #",
                " ###   ###   ###   ###   ###   ###         ###             ",
                "#   # #   # #     #   # #     #     #   # #   # #     #   #",
                "#   # #   # #     #   # #     #     #   # #   # #     #   #",
                "       ###   ###   ###   ###         ###               ### ",
            ],
        );

        test_segments(
            "lnopqrstuy",
            &[
                "                   ###   ###         ###                   ",
                "#                 #   # #   #       #     #           #   #",
                "#                 #   # #   #       #     #           #   #",
                "       ###   ###   ###   ###   ###   ###   ###         ### ",
                "#     #   # #   # #         # #         # #     #   #     #",
                "#     #   # #   # #         # #         # #     #   #     #",
                " ###         ###                     ###   ###   ###   ### ",
            ],
        );
    }

    #[test]
    fn upper_case_letters() {
        test_segments(
            "ABCDEFGHIJ",
            &[
                " ###         ###         ###   ###   ###                   ",
                "#   # #     #         # #     #     #     #   # #         #",
                "#   # #     #         # #     #     #     #   # #         #",
                " ###   ###         ###   ###   ###         ###             ",
                "#   # #   # #     #   # #     #     #   # #   # #     #   #",
                "#   # #   # #     #   # #     #     #   # #   # #     #   #",
                "       ###   ###   ###   ###         ###               ### ",
            ],
        );

        test_segments(
            "LNOPQRSTUY",
            &[
                "             ###   ###   ###         ###                   ",
                "#           #   # #   # #   #       #     #     #   # #   #",
                "#           #   # #   # #   #       #     #     #   # #   #",
                "       ###         ###   ###   ###   ###   ###         ### ",
                "#     #   # #   # #         # #         # #     #   #     #",
                "#     #   # #   # #         # #         # #     #   #     #",
                " ###         ###                     ###   ###   ###   ### ",
            ],
        );
    }

    #[test]
    fn other_chars() {
        test_segments(
            " _-=°\"'",
            &[
                "                         ###             ",
                "                        #   # #   # #    ",
                "                        #   # #   # #    ",
                "             ###   ###   ###             ",
                "                                         ",
                "                                         ",
                "       ###         ###                   ",
            ],
        );

        test_segments(
            "([])?",
            &[
                " ###   ###   ###   ###   ### ",
                "#     #         #     #     #",
                "#     #         #     #     #",
                "                         ### ",
                "#     #         #     # #    ",
                "#     #         #     # #    ",
                " ###   ###   ###   ###       ",
            ],
        );
    }

    #[test]
    fn private_use_area() {
        test_segments(
            "\u{E040}\u{E020}\u{E010}\u{E008}\u{E004}\u{E002}\u{E001}\u{E055}\u{E02A}",
            &[
                " ###                                       ###       ",
                "          #                   #                 #   #",
                "          #                   #                 #   #",
                "                                     ###   ###       ",
                "                #       #                 #   #      ",
                "                #       #                 #   #      ",
                "                   ###                           ### ",
            ],
        );
    }
}
