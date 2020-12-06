use core::convert::TryFrom;
use embedded_graphics::{
    geometry::AnchorPoint, prelude::*, primitives::Rectangle, style::TextStyle,
};

use crate::{segment::Segment, Segments};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[non_exhaustive]
pub struct SevenSegmentTextStyle<C> {
    pub digit_size: Size,
    pub digit_spacing: u32,
    pub segment_width: u32,
    pub segment_color: Option<C>,
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
}

impl<C: PixelColor> TextStyle for SevenSegmentTextStyle<C> {
    type Color = C;

    fn render_line<D>(
        &self,
        text: &str,
        mut position: Point,
        target: &mut D,
    ) -> Result<Point, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
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

        // TODO: make row spacing configurable
        Ok(Point::new(0, self.digit_size.height as i32 * 3 / 2))
    }

    fn line_bounding_box(&self, _text: &str, _position: Point) -> (Rectangle, Point) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SevenSegmentTextStyleBuilder;
    use embedded_graphics::{fonts::Text, mock_display::MockDisplay, pixelcolor::BinaryColor};

    fn test_digits(
        style: SevenSegmentTextStyle<BinaryColor>,
        digits: &str,
        expected_pattern: &[&str],
    ) {
        let mut display = MockDisplay::new();
        Text::new(digits, Point::zero())
            .into_styled(style)
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
}
