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
    /// Segment bit field.
    ///
    /// The `Segments` bit field is used to define the active segments in a seven segment digit.
    /// `Segments` can be constructed converting a [`char`] or by combining the `A`-`G` constants.
    ///
    /// Use [`Digit`](crate::Digit) to draw a single digit from a `Segments` bit field or convert
    /// the `Segments` bit field into a [`char`] to use it in a [`Text`](embedded_graphics::text::Text).
    ///
    /// # Examples
    ///
    /// ```
    /// use eg_seven_segment::Segments;
    ///
    /// let segments_from_char = Segments::try_from('1').unwrap();
    /// let segments_from_consts = Segments::B | Segments::C;
    /// assert_eq!(segments_from_char, segments_from_consts);
    /// ```
    ///
    /// All `Segments` values can be converted into a `char` to include them in a string:
    ///
    /// ```
    /// use eg_seven_segment::Segments;
    ///
    /// let as_char = char::from(Segments::B | Segments::C);
    ///
    /// // The returned char is guaranteed to be convertible into the same `Segments` bit field:
    /// assert_eq!(Segments::try_from(as_char).unwrap(), Segments::B | Segments::C);
    ///
    /// // But the actual value of the char isn't defined and the application shouldn't depend
    /// // on the returned value. While the char '1' can be displayed using segments B and C
    /// // this assertion could fail:
    /// // assert_eq!(as_char, '1');
    /// ```
    ///
    /// # Segment layout
    ///
    /// <center>
    /// <svg width="108" height="194" version="1.1" viewBox="0 0 28.575 51.329" xmlns="http://www.w3.org/2000/svg">
    /// <g transform="translate(-51.329 -5.0271)">
    /// <path d="m51.594 11.377 2.6458-2.6458 2.6458 2.6458v15.875l-2.6458 2.6458-2.6458-2.6458z" fill="#e5e5e5" stop-color="#000000" stroke="#b2b2b2" stroke-linecap="square" stroke-width=".52917" style="-inkscape-stroke:none;font-variation-settings:normal"/>
    /// <path d="m73.554 5.2917 2.6458 2.6458-2.6458 2.6458h-15.875l-2.6458-2.6458 2.6458-2.6458z" fill="#e5e5e5" stroke="#b2b2b2" stroke-linecap="square" stroke-width=".52917"/>
    /// <path d="m73.554 28.046 2.6458 2.6458-2.6458 2.6458h-15.875l-2.6458-2.6458 2.6458-2.6458z" fill="#e5e5e5" stop-color="#000000" stroke="#b2b2b2" stroke-linecap="square" stroke-width=".52917" style="-inkscape-stroke:none;font-variation-settings:normal"/>
    /// <path d="m79.64 27.252-2.6458 2.6458-2.6458-2.6458v-15.875l2.6458-2.6458 2.6458 2.6458z" fill="#e5e5e5" stop-color="#000000" stroke="#b2b2b2" stroke-linecap="square" stroke-width=".52917" style="-inkscape-stroke:none;font-variation-settings:normal"/>
    /// <path d="m51.594 34.131 2.6458-2.6458 2.6458 2.6458v15.875l-2.6458 2.6458-2.6458-2.6458z" fill="#e5e5e5" stop-color="#000000" stroke="#b2b2b2" stroke-linecap="square" stroke-width=".52917" style="-inkscape-stroke:none;font-variation-settings:normal"/>
    /// <path d="m73.554 50.8 2.6458 2.6458-2.6458 2.6458h-15.875l-2.6458-2.6458 2.6458-2.6458z" fill="#e5e5e5" stop-color="#000000" stroke="#b2b2b2" stroke-linecap="square" stroke-width=".52917" style="-inkscape-stroke:none;font-variation-settings:normal"/>
    /// <path d="m79.64 50.006-2.6458 2.6458-2.6458-2.6458v-15.875l2.6458-2.6458 2.6458 2.6458z" fill="#e5e5e5" stop-color="#000000" stroke="#b2b2b2" stroke-linecap="square" stroke-width=".52917" style="-inkscape-stroke:none;font-variation-settings:normal"/>
    /// <path d="m75.923 20.854v-3.0788h1.0764q0.32171 0 0.53067 0.11124 0.20896 0.10974 0.31118 0.29615 0.10223 0.18491 0.10223 0.4104 0 0.19844-0.07066 0.32772-0.06915 0.12928-0.1834 0.20445-0.11275 0.07517-0.24504 0.11124v0.03007q0.14131 9e-3 0.28413 0.09922 0.14282 0.0902 0.23903 0.25857 0.09621 0.16837 0.09621 0.41191 0 0.23151-0.10523 0.41642-0.10523 0.18491-0.33223 0.29315-0.227 0.10824-0.5908 0.10824zm0.37282-0.33073h0.73963q0.3653 0 0.51864-0.14131 0.15484-0.14282 0.15484-0.34576 0-0.15634-0.07968-0.28864-0.07967-0.1338-0.227-0.21347-0.14732-0.08118-0.34877-0.08118h-0.75767zm0-1.3951h0.69152q0.16837 0 0.30367-0.06615 0.1368-0.06615 0.21648-0.18641 0.08118-0.12026 0.08118-0.28262 0-0.20295-0.14131-0.34426-0.14131-0.14281-0.44799-0.14281h-0.70355z"/>
    /// <path d="m78.293 41.491h-0.37282q-0.03307-0.16086-0.11576-0.28262-0.08118-0.12177-0.19844-0.20445-0.11576-0.08418-0.25707-0.12628-0.14131-0.04209-0.29465-0.04209-0.27962 0-0.50662 0.14131-0.2255 0.14131-0.35929 0.41642-0.13229 0.27511-0.13229 0.67499t0.13229 0.67499q0.1338 0.27511 0.35929 0.41642 0.227 0.14131 0.50662 0.14131 0.15334 0 0.29465-0.04209 0.14131-0.04209 0.25707-0.12478 0.11726-0.08419 0.19844-0.20595 0.08268-0.12327 0.11576-0.28262h0.37282q-0.04209 0.23602-0.15334 0.42243-0.11125 0.18641-0.27661 0.3172-0.16536 0.12928-0.37132 0.19693-0.20445 0.06765-0.43746 0.06765-0.39387 0-0.70054-0.19242-0.30668-0.19242-0.48256-0.54721t-0.17589-0.84186 0.17589-0.84186 0.48256-0.54721q0.30668-0.19242 0.70054-0.19242 0.23301 0 0.43746 0.06765 0.20595 0.06765 0.37132 0.19844 0.16536 0.12928 0.27661 0.3157 0.11124 0.18491 0.15334 0.42243z"/>
    /// <path d="m65.358 54.985h-0.95009v-3.0788h0.99219q0.44799 0 0.76669 0.18491 0.3187 0.1834 0.48858 0.52766 0.16987 0.34276 0.16987 0.82081 0 0.48106-0.17138 0.82832-0.17138 0.34576-0.4991 0.53217-0.32772 0.18491-0.79676 0.18491zm-0.57727-0.33073h0.55322q0.38184 0 0.6329-0.14732 0.25105-0.14732 0.37432-0.41942 0.12327-0.2721 0.12327-0.64793 0-0.37282-0.12177-0.64192-0.12177-0.2706-0.3638-0.41492-0.24203-0.14582-0.60283-0.14582h-0.59531z"/>
    /// <path d="m53.299 43.608v-3.0788h1.8581v0.33073h-1.4853v1.0403h1.3891v0.33073h-1.3891v1.0463h1.5093v0.33073z"/>
    /// <path d="m53.317 20.854v-3.0788h1.8461v0.33073h-1.4732v1.0403h1.3349v0.33073h-1.3349v1.377z"/>
    /// <path d="m66.528 30.114q-0.04961-0.15184-0.13079-0.2721-0.07968-0.12177-0.19092-0.20746-0.10974-0.08569-0.24955-0.13079-0.13981-0.0451-0.30668-0.0451-0.2736 0-0.4976 0.14131-0.22399 0.14131-0.35628 0.41642-0.13229 0.27511-0.13229 0.67499t0.1338 0.67499q0.13379 0.27511 0.3623 0.41642 0.2285 0.14131 0.51413 0.14131 0.26458 0 0.46603-0.11275 0.20295-0.11425 0.3157-0.32171 0.11425-0.20896 0.11425-0.49158l0.11425 0.02405h-0.92604v-0.33073h1.1726v0.33073q0 0.38034-0.16236 0.66146-0.16085 0.28112-0.44498 0.43596-0.28262 0.15334-0.64943 0.15334-0.4089 0-0.71858-0.19242-0.30818-0.19242-0.48106-0.54721-0.17138-0.35478-0.17138-0.84186 0-0.48707 0.17138-0.84186 0.17288-0.35478 0.47655-0.54721 0.30517-0.19242 0.69904-0.19242 0.24354 0 0.454 0.07366 0.21197 0.07216 0.37733 0.20595 0.16687 0.13229 0.27811 0.3172 0.11124 0.1834 0.15334 0.4074z"/>
    /// <path d="m64.685 9.4769h-0.39086l1.1305-3.0788h0.38485l1.1305 3.0788h-0.39086l-0.92003-2.5917h-0.02405zm0.14432-1.2027h1.5755v0.33073h-1.5755z"/>
    /// </g>
    /// </svg>
    /// </center>
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

impl From<Segments> for char {
    fn from(segments: Segments) -> Self {
        char::from_u32(0xE000 + u32::from(segments.bits())).unwrap_or(' ')
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
