use core::convert::TryFrom;

use embedded_graphics::{
    geometry::AnchorPoint,
    prelude::*,
    primitives::Rectangle,
    text::{
        renderer::{TextMetrics, TextRenderer},
        Baseline,
    },
};

use crate::{segment::Segment, Segments};

/// Seven-segment character style.
///
/// Use [`SevenSegmentStyleBuilder`] to build styles.
///
/// [`SevenSegmentStyleBuilder`]: struct.SevenSegmentStyleBuilder.html
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct SevenSegmentStyle<C> {
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

impl<C: PixelColor> SevenSegmentStyle<C> {
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

impl<C: PixelColor> TextRenderer for SevenSegmentStyle<C> {
    type Color = C;

    fn draw_string<D>(
        &self,
        text: &str,
        mut position: Point,
        baseline: Baseline,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
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
                }

                position += Size::new(self.segment_width + self.digit_spacing, 0);
            } else if c == '.' {
                if let Some(color) = self.segment_color {
                    let rect = Rectangle::new(
                        position + Size::new(0, self.digit_size.height - self.segment_width),
                        Size::new(self.segment_width, self.segment_width),
                    );
                    target.fill_solid(&rect, color)?;
                }

                position += Size::new(self.segment_width + self.digit_spacing, 0);
            } else {
                // TODO: how should other characters be handled?
            }
        }

        position += Size::new(0, self.baseline_offset(baseline));

        Ok(position)
    }

    fn draw_whitespace<D>(
        &self,
        width: u32,
        position: Point,
        _baseline: Baseline,
        _target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        Ok(position + Size::new(width, 0))
    }

    fn measure_string(&self, text: &str, position: Point, baseline: Baseline) -> TextMetrics {
        let width = text
            .chars()
            .map(|c| {
                let width = if c == '.' || c == ':' {
                    self.segment_width
                } else {
                    self.digit_size.width
                };

                width + self.digit_spacing
            })
            .sum::<u32>()
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
    use crate::SevenSegmentStyleBuilder;
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor, text::Text};

    fn test_digits(
        character_style: SevenSegmentStyle<BinaryColor>,
        digits: &str,
        expected_pattern: &[&str],
    ) {
        let mut display = MockDisplay::new();

        Text::with_baseline(digits, Point::zero(), character_style, Baseline::Top)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(expected_pattern);
    }

    #[test]
    fn digits_1px_9px() {
        let style = SevenSegmentStyleBuilder::new()
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
        let style = SevenSegmentStyleBuilder::new()
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
        let style = SevenSegmentStyleBuilder::new()
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
        let style = SevenSegmentStyleBuilder::new()
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
        let style = SevenSegmentStyleBuilder::new()
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
        let style = SevenSegmentStyleBuilder::new()
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
        let style1 = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(5, 9))
            .digit_spacing(1)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        let style2 = SevenSegmentStyleBuilder::from(&style1)
            .digit_size(Size::new(7, 11))
            .segment_color(BinaryColor::Off)
            .build();

        let mut display = MockDisplay::new();
        let p = Point::new(0, 10);
        let next = Text::new("12", p, style1).draw(&mut display).unwrap();
        Text::new("3", next, style2).draw(&mut display).unwrap();

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
    fn chaining_with_colon() {
        let style1 = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(5, 9))
            .digit_spacing(1)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        let style2 = SevenSegmentStyleBuilder::from(&style1)
            .digit_size(Size::new(7, 11))
            .segment_color(BinaryColor::Off)
            .build();

        let mut display = MockDisplay::new();
        let p = Point::new(0, 10);
        let next = Text::new("1:2", p, style1).draw(&mut display).unwrap();
        Text::new("3", next, style2).draw(&mut display).unwrap();

        display.assert_pattern(&[
            "               ..... ",
            "                    .",
            "         ###        .",
            "    #       #       .",
            "    #       #       .",
            "    # #     #  ..... ",
            "         ###        .",
            "    #   #           .",
            "    # # #           .",
            "    #   #           .",
            "         ###   ..... ",
        ])
    }

    #[test]
    fn multiple_lines() {
        let style = SevenSegmentStyleBuilder::new()
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
        let style = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(5, 9))
            .digit_spacing(2)
            .segment_width(1)
            .segment_color(BinaryColor::On)
            .build();

        let mut display = MockDisplay::new();
        Text::with_baseline("8", Point::new(0, 8), style, baseline)
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

    #[test]
    fn measure_string() {
        let style = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(7, 12))
            .digit_spacing(1)
            .segment_width(2)
            .segment_color(BinaryColor::On)
            .build();

        let position = Point::new(1, 2);

        let metrics = style.measure_string("12", position, Baseline::Top);
        assert_eq!(
            metrics.bounding_box,
            Rectangle::new(
                position,
                style.digit_size.component_mul(Size::new(2, 1)) + Size::new(style.digit_spacing, 0)
            )
        );
        assert_eq!(
            metrics.next_position,
            position + metrics.bounding_box.size.x_axis()
        );
    }

    #[test]
    fn measure_string_with_colon() {
        let style = SevenSegmentStyleBuilder::new()
            .digit_size(Size::new(7, 12))
            .digit_spacing(1)
            .segment_width(2)
            .segment_color(BinaryColor::On)
            .build();

        let position = Point::new(1, 2);

        let metrics = style.measure_string("1:2", position, Baseline::Top);
        assert_eq!(
            metrics.bounding_box,
            Rectangle::new(
                position,
                style.digit_size.component_mul(Size::new(2, 1))
                    + Size::new(style.digit_spacing * 2 + style.segment_width, 0)
            )
        );
        assert_eq!(
            metrics.next_position,
            position + metrics.bounding_box.size.x_axis()
        );
    }
}
