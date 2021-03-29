use core::convert::TryFrom;

use embedded_graphics::{
    geometry::AnchorPoint,
    prelude::*,
    primitives::Rectangle,
    text::{
        renderer::{CharacterStyle, RenderText, TextMetrics},
        Baseline,
    },
};

use crate::{segment::Segment, Segments};

/// Seven-segment text style.
///
/// Use [`SevenSegmentTextStyleBuilder`] to build styles.
///
/// [`SevenSegmentTextStyleBuilder`]: struct.SevenSegmentTextStyleBuilder.html
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct SevenSegmentTextStyle<C> {
    /// The size of each digit.
    pub digit_size: Size,

    /// The spacing between adjacent digits.
    pub digit_spacing: u32,

    /// The width of the segments.
    pub segment_width: u32,

    /// The color of active segments.
    pub segment_color: Option<C>,

    /// The color of inactive segments.
    pub inactive_segment_color: Option<C>,
}

impl<C: PixelColor> SevenSegmentTextStyle<C> {
    /// Returns the fill color for the given segment state.
    fn state_color(&self, state: bool) -> Option<C> {
        if state {
            self.segment_color
        } else {
            self.inactive_segment_color
        }
    }

    /// Returns the vertical offset between the line position and the top edge of the bounding box.
    fn baseline_offset(&self, baseline: Baseline) -> u32 {
        let bottom = self.digit_size.height.saturating_sub(1);

        match baseline {
            Baseline::Top => 0,
            Baseline::Bottom | Baseline::Alphabetic => bottom,
            Baseline::Middle => bottom / 2,
        }
    }
}

impl<D: DrawTarget> RenderText<D> for SevenSegmentTextStyle<D::Color> {
    fn draw_string(
        &self,
        text: &str,
        mut position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error> {
        position -= Size::new(0, self.baseline_offset(baseline));

        for c in text.chars() {
            if let Ok(segments) = Segments::try_from(c) {
                let rect = Rectangle::new(position, self.digit_size);

                let vertical_size = Size::new(self.digit_size.width, self.segment_width);
                let horizontal_size_top = Size::new(
                    self.segment_width,
                    (self.digit_size.height + self.segment_width) / 2,
                );
                let horizontal_size_bottom = Size::new(
                    self.segment_width,
                    (self.digit_size.height + self.segment_width + 1) / 2,
                );

                if let Some(color) = self.state_color(segments.contains(Segments::A)) {
                    Segment::with_reduced_size(
                        rect.resized(vertical_size, AnchorPoint::TopLeft),
                        color,
                    )
                    .draw(target)?;
                }

                if let Some(color) = self.state_color(segments.contains(Segments::B)) {
                    Segment::with_reduced_size(
                        rect.resized(horizontal_size_top, AnchorPoint::TopRight),
                        color,
                    )
                    .draw(target)?;
                }

                if let Some(color) = self.state_color(segments.contains(Segments::C)) {
                    Segment::with_reduced_size(
                        rect.resized(horizontal_size_bottom, AnchorPoint::BottomRight),
                        color,
                    )
                    .draw(target)?;
                }

                if let Some(color) = self.state_color(segments.contains(Segments::D)) {
                    Segment::with_reduced_size(
                        rect.resized(vertical_size, AnchorPoint::BottomLeft),
                        color,
                    )
                    .draw(target)?;
                }

                if let Some(color) = self.state_color(segments.contains(Segments::E)) {
                    Segment::with_reduced_size(
                        rect.resized(horizontal_size_bottom, AnchorPoint::BottomLeft),
                        color,
                    )
                    .draw(target)?;
                }

                if let Some(color) = self.state_color(segments.contains(Segments::F)) {
                    Segment::with_reduced_size(
                        rect.resized(horizontal_size_top, AnchorPoint::TopLeft),
                        color,
                    )
                    .draw(target)?;
                }

                if let Some(color) = self.state_color(segments.contains(Segments::G)) {
                    Segment::with_reduced_size(
                        rect.resized(vertical_size, AnchorPoint::CenterLeft),
                        color,
                    )
                    .draw(target)?;
                }

                position += self.digit_size.x_axis() + Size::new(self.digit_spacing, 0);
            } else if c == ':' {
                if let Some(color) = self.segment_color {
                    let dy = self.digit_size.height / 3;

                    let mut rect = Rectangle::new(
                        position + Size::new(0, dy - self.segment_width / 2),
                        Size::new(self.segment_width, self.segment_width),
                    );
                    target.fill_solid(&rect, color)?;

                    rect.top_left += Size::new(0, dy);
                    target.fill_solid(&rect, color)?;

                    position += Size::new(self.segment_width + self.digit_spacing, 0);
                }
            } else {
                // TODO: add '.'
                // TODO: how should other characters be handled?
            }
        }

        position += Size::new(0, self.baseline_offset(baseline));

        Ok(position)
    }

    fn draw_whitespace(
        &self,
        width: u32,
        position: Point,
        _baseline: Baseline,
        _target: &mut D,
    ) -> Result<Point, D::Error> {
        Ok(position + Size::new(width, 0))
    }
}

impl<C: PixelColor> CharacterStyle for SevenSegmentTextStyle<C> {
    type Color = C;

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        let width = (text.len() as u32 * (self.digit_size.width + self.digit_spacing))
            .saturating_sub(self.digit_spacing);

        let bounding_box = Rectangle::new(
            position - Size::new(0, self.baseline_offset(baseline)),
            Size::new(width, self.digit_size.height),
        );
        let next_position = position + Size::new(width, 0);

        TextMetrics {
            bounding_box,
            next_position,
        }
    }

    fn line_height(&self) -> u32 {
        self.digit_size.height + self.digit_spacing
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SevenSegmentTextStyleBuilder;
    use embedded_graphics::{
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        text::{Text, TextStyleBuilder},
    };

    fn test_digits(
        character_style: SevenSegmentTextStyle<BinaryColor>,
        digits: &str,
        expected_pattern: &[&str],
    ) {
        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(Baseline::Top)
            .build();

        let mut display = MockDisplay::new();

        Text::new(digits, Point::zero())
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(expected_pattern);
    }

    #[test]
    fn digits_1px_9px() {
        let style = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(5, 9))
            .digit_spacing(1)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        test_digits(
            style,
            "0123456789",
            &[
                " ###         ###   ###         ###   ###   ###   ###   ### ",
                "#   #     #     #     # #   # #     #         # #   # #   #",
                "#   #     #     #     # #   # #     #         # #   # #   #",
                "#   #     #     #     # #   # #     #         # #   # #   #",
                "             ###   ###   ###   ###   ###         ###   ### ",
                "#   #     # #         #     #     # #   #     # #   #     #",
                "#   #     # #         #     #     # #   #     # #   #     #",
                "#   #     # #         #     #     # #   #     # #   #     #",
                " ###         ###   ###         ###   ###         ###   ### ",
            ],
        );
    }

    #[test]
    fn digits_1px_10px() {
        let style = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(5, 10))
            .digit_spacing(1)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        test_digits(
            style,
            "0123456789",
            &[
                " ###         ###   ###         ###   ###   ###   ###   ### ",
                "#   #     #     #     # #   # #     #         # #   # #   #",
                "#   #     #     #     # #   # #     #         # #   # #   #",
                "#   #     #     #     # #   # #     #         # #   # #   #",
                "             ###   ###   ###   ###   ###         ###   ### ",
                "#   #     # #         #     #     # #   #     # #   #     #",
                "#   #     # #         #     #     # #   #     # #   #     #",
                "#   #     # #         #     #     # #   #     # #   #     #",
                "#   #     # #         #     #     # #   #     # #   #     #",
                " ###         ###   ###         ###   ###         ###   ### ",
            ],
        );
    }

    #[test]
    fn digits_2px_12px() {
        let style = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(7, 12))
            .digit_spacing(1)
            .segment_width(2)
            .segment_color(BinaryColor::On)
            .build();

        test_digits(
            style,
            "01234",
            &[
                "  ###             ###     ###          ",
                "  ###             ###     ###          ",
                "##   ##      ##      ##      ## ##   ##",
                "##   ##      ##      ##      ## ##   ##",
                "##   ##      ##      ##      ## ##   ##",
                "                  ###     ###     ###  ",
                "                  ###     ###     ###  ",
                "##   ##      ## ##           ##      ##",
                "##   ##      ## ##           ##      ##",
                "##   ##      ## ##           ##      ##",
                "  ###             ###     ###          ",
                "  ###             ###     ###          ",
            ],
        );

        test_digits(
            style,
            "56789",
            &[
                "  ###     ###     ###     ###     ###  ",
                "  ###     ###     ###     ###     ###  ",
                "##      ##           ## ##   ## ##   ##",
                "##      ##           ## ##   ## ##   ##",
                "##      ##           ## ##   ## ##   ##",
                "  ###     ###             ###     ###  ",
                "  ###     ###             ###     ###  ",
                "     ## ##   ##      ## ##   ##      ##",
                "     ## ##   ##      ## ##   ##      ##",
                "     ## ##   ##      ## ##   ##      ##",
                "  ###     ###             ###     ###  ",
                "  ###     ###             ###     ###  ",
            ],
        );
    }

    #[test]
    fn digits_2px_13px() {
        let style = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(7, 13))
            .digit_spacing(1)
            .segment_width(2)
            .segment_color(BinaryColor::On)
            .build();

        test_digits(
            style,
            "01234",
            &[
                "  ###             ###     ###          ",
                "  ###             ###     ###          ",
                "##   ##      ##      ##      ## ##   ##",
                "##   ##      ##      ##      ## ##   ##",
                "##   ##      ##      ##      ## ##   ##",
                "                  ###     ###     ###  ",
                "                  ###     ###     ###  ",
                "##   ##      ## ##           ##      ##",
                "##   ##      ## ##           ##      ##",
                "##   ##      ## ##           ##      ##",
                "##   ##      ## ##           ##      ##",
                "  ###             ###     ###          ",
                "  ###             ###     ###          ",
            ],
        );

        test_digits(
            style,
            "56789",
            &[
                "  ###     ###     ###     ###     ###  ",
                "  ###     ###     ###     ###     ###  ",
                "##      ##           ## ##   ## ##   ##",
                "##      ##           ## ##   ## ##   ##",
                "##      ##           ## ##   ## ##   ##",
                "  ###     ###             ###     ###  ",
                "  ###     ###             ###     ###  ",
                "     ## ##   ##      ## ##   ##      ##",
                "     ## ##   ##      ## ##   ##      ##",
                "     ## ##   ##      ## ##   ##      ##",
                "     ## ##   ##      ## ##   ##      ##",
                "  ###     ###             ###     ###  ",
                "  ###     ###             ###     ###  ",
            ],
        );
    }

    #[test]
    fn digits_3px_15px() {
        let style = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(9, 15))
            .digit_spacing(1)
            .segment_width(3)
            .segment_color(BinaryColor::On)
            .build();

        test_digits(
            style,
            "01234",
            &[
                "   ###                 ###       ###             ",
                "  #####               #####     #####            ",
                " # ### #         #     ### #     ### #   #     # ",
                "###   ###       ###       ###       ### ###   ###",
                "###   ###       ###       ###       ### ###   ###",
                "###   ###       ###       ###       ### ###   ###",
                " #     #         #     ### #     ### #   # ### # ",
                "                      #####     #####     #####  ",
                " #     #         #   # ###       ### #     ### # ",
                "###   ###       ### ###             ###       ###",
                "###   ###       ### ###             ###       ###",
                "###   ###       ### ###             ###       ###",
                " # ### #         #   # ###       ### #         # ",
                "  #####               #####     #####            ",
                "   ###                 ###       ###             ",
            ],
        );

        test_digits(
            style,
            "56789",
            &[
                "   ###       ###       ###       ###       ###   ",
                "  #####     #####     #####     #####     #####  ",
                " # ###     # ###       ### #   # ### #   # ### # ",
                "###       ###             ### ###   ### ###   ###",
                "###       ###             ### ###   ### ###   ###",
                "###       ###             ### ###   ### ###   ###",
                " # ###     # ###           #   # ### #   # ### # ",
                "  #####     #####               #####     #####  ",
                "   ### #   # ### #         #   # ### #     ### # ",
                "      ### ###   ###       ### ###   ###       ###",
                "      ### ###   ###       ### ###   ###       ###",
                "      ### ###   ###       ### ###   ###       ###",
                "   ### #   # ### #         #   # ### #     ### # ",
                "  #####     #####               #####     #####  ",
                "   ###       ###                 ###       ###   ",
            ],
        );
    }

    #[test]
    fn digits_3px_16px() {
        let style = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(9, 16))
            .digit_spacing(1)
            .segment_width(3)
            .segment_color(BinaryColor::On)
            .build();

        test_digits(
            style,
            "01234",
            &[
                "   ###                 ###       ###             ",
                "  #####               #####     #####            ",
                " # ### #         #     ### #     ### #   #     # ",
                "###   ###       ###       ###       ### ###   ###",
                "###   ###       ###       ###       ### ###   ###",
                "###   ###       ###       ###       ### ###   ###",
                " #     #         #     ### #     ### #   # ### # ",
                "                      #####     #####     #####  ",
                " #     #         #   # ###       ### #     ### # ",
                "###   ###       ### ###             ###       ###",
                "###   ###       ### ###             ###       ###",
                "###   ###       ### ###             ###       ###",
                "###   ###       ### ###             ###       ###",
                " # ### #         #   # ###       ### #         # ",
                "  #####               #####     #####            ",
                "   ###                 ###       ###             ",
            ],
        );

        test_digits(
            style,
            "56789",
            &[
                "   ###       ###       ###       ###       ###   ",
                "  #####     #####     #####     #####     #####  ",
                " # ###     # ###       ### #   # ### #   # ### # ",
                "###       ###             ### ###   ### ###   ###",
                "###       ###             ### ###   ### ###   ###",
                "###       ###             ### ###   ### ###   ###",
                " # ###     # ###           #   # ### #   # ### # ",
                "  #####     #####               #####     #####  ",
                "   ### #   # ### #         #   # ### #     ### # ",
                "      ### ###   ###       ### ###   ###       ###",
                "      ### ###   ###       ### ###   ###       ###",
                "      ### ###   ###       ### ###   ###       ###",
                "      ### ###   ###       ### ###   ###       ###",
                "   ### #   # ### #         #   # ### #     ### # ",
                "  #####     #####               #####     #####  ",
                "   ###       ###                 ###       ###   ",
            ],
        );
    }

    #[test]
    fn chaining() {
        let style1 = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(5, 9))
            .digit_spacing(1)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        let style2 = SevenSegmentTextStyleBuilder::from(&style1)
            .digit_size(Size::new(7, 11))
            .segment_color(BinaryColor::Off)
            .build();

        let mut display = MockDisplay::new();
        let next = Text::new("12", Point::new(0, 10))
            .into_styled(style1)
            .draw(&mut display)
            .unwrap();
        Text::new("3", next)
            .into_styled(style2)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            "             ..... ",
            "                  .",
            "       ###        .",
            "    #     #       .",
            "    #     #       .",
            "    #     #  ..... ",
            "       ###        .",
            "    # #           .",
            "    # #           .",
            "    # #           .",
            "       ###   ..... ",
        ])
    }

    #[test]
    fn multiple_lines() {
        let style = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(5, 9))
            .digit_spacing(2)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        test_digits(
            style,
            "12\n3",
            &[
                "        ### ",
                "    #      #",
                "    #      #",
                "    #      #",
                "        ### ",
                "    #  #    ",
                "    #  #    ",
                "    #  #    ",
                "        ### ",
                "            ",
                "            ",
                " ###        ",
                "    #       ",
                "    #       ",
                "    #       ",
                " ###        ",
                "    #       ",
                "    #       ",
                "    #       ",
                " ###        ",
            ],
        );
    }

    fn test_baseline(baseline: Baseline, expected_pattern: &[&str]) {
        let character_style = SevenSegmentTextStyleBuilder::new()
            .digit_size(Size::new(5, 9))
            .digit_spacing(2)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        let text_style = TextStyleBuilder::new()
            .character_style(character_style)
            .baseline(baseline)
            .build();

        let mut display = MockDisplay::new();
        Text::new("8", Point::new(0, 8))
            .into_styled(text_style)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(expected_pattern);
    }

    #[test]
    fn baseline_top() {
        test_baseline(
            Baseline::Top,
            &[
                "     ", //
                "     ", //
                "     ", //
                "     ", //
                "     ", //
                "     ", //
                "     ", //
                "     ", //
                " ### ", // ###
                "#   #", //
                "#   #", //
                "#   #", //
                " ### ", //
                "#   #", //
                "#   #", //
                "#   #", //
                " ### ", //
            ],
        );
    }

    #[test]
    fn baseline_middle() {
        test_baseline(
            Baseline::Middle,
            &[
                "     ", //
                "     ", //
                "     ", //
                "     ", //
                " ### ", //
                "#   #", //
                "#   #", //
                "#   #", //
                " ### ", // ###
                "#   #", //
                "#   #", //
                "#   #", //
                " ### ", //
            ],
        );
    }

    #[test]
    fn baseline_bottom() {
        test_baseline(
            Baseline::Bottom,
            &[
                " ### ", //
                "#   #", //
                "#   #", //
                "#   #", //
                " ### ", //
                "#   #", //
                "#   #", //
                "#   #", //
                " ### ", // ###
            ],
        );
    }

    #[test]
    fn baseline_alphabetic() {
        test_baseline(
            Baseline::Alphabetic,
            &[
                " ### ", //
                "#   #", //
                "#   #", //
                "#   #", //
                " ### ", //
                "#   #", //
                "#   #", //
                "#   #", //
                " ### ", // ###
            ],
        );
    }
}
