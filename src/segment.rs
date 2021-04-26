use embedded_graphics::{prelude::*, primitives::Rectangle};

/// Segment drawable.
pub struct Segment<C> {
    rect: Rectangle,
    color: C,
}

impl<C> Segment<C> {
    /// Creates a new segment drawable.
    pub fn new(rect: Rectangle, color: C) -> Self {
        Self { rect, color }
    }

    /// Creates a new segment drawable with reduced size.
    ///
    /// The size of the rectangle is reduced so that a vertical and horizontal segment with the
    /// same top left corner don't overlap.
    pub fn with_reduced_size(mut rect: Rectangle, color: C) -> Self {
        // TODO: handle rects that are too small
        if rect.size.width > rect.size.height {
            let size_offset = rect.size.height / 2 + 1;
            rect.top_left += Size::new(size_offset, 0);
            rect.size.width -= 2 * size_offset;
        } else {
            let size_offset = rect.size.width / 2 + 1;
            rect.top_left += Size::new(0, size_offset);
            rect.size.height -= 2 * size_offset;
        }

        Self::new(rect, color)
    }
}

impl<C: PixelColor> Drawable for Segment<C> {
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        if self.rect.is_zero_sized() {
            return Ok(());
        }

        let center_2x = self.rect.top_left * 2 + (self.rect.size - Size::new(1, 1));

        if self.rect.size.width > self.rect.size.height {
            // Draw horizontal segment.
            for y in self.rect.rows() {
                let offset = (y * 2 - center_2x.y).abs() / 2;

                let scanline = Rectangle::new(
                    Point::new(self.rect.top_left.x + offset, y),
                    Size::new(self.rect.size.width - offset as u32 * 2, 1),
                );

                target.fill_solid(&scanline, self.color)?;
            }
        } else {
            // Draw vertical segment.
            for x in self.rect.columns() {
                let offset = (x * 2 - center_2x.x).abs() / 2;

                let scanline = Rectangle::new(
                    Point::new(x, self.rect.top_left.y + offset),
                    Size::new(1, self.rect.size.height - offset as u32 * 2),
                );

                target.fill_solid(&scanline, self.color)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor};

    fn test_segment(rect: Rectangle, expected_pattern: &[&str]) {
        let mut display = MockDisplay::new();
        Segment::new(rect, BinaryColor::On)
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&expected_pattern);
    }

    #[test]
    fn horizontal_1px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(10, 1)),
            &["##########"],
        );
    }

    #[test]
    fn horizontal_2px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(10, 2)),
            &[
                "##########", //
                "##########", //
            ],
        );
    }

    #[test]
    fn horizontal_3px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(10, 3)),
            &[
                " ######## ", //
                "##########", //
                " ######## ", //
            ],
        );
    }

    #[test]
    fn horizontal_4px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(10, 4)),
            &[
                " ######## ", //
                "##########", //
                "##########", //
                " ######## ", //
            ],
        );
    }

    #[test]
    fn horizontal_5px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(10, 5)),
            &[
                "  ######  ", //
                " ######## ", //
                "##########", //
                " ######## ", //
                "  ######  ", //
            ],
        );
    }

    #[test]
    fn vertical_1px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(1, 10)),
            &[
                "#", //
                "#", //
                "#", //
                "#", //
                "#", //
                "#", //
                "#", //
                "#", //
                "#", //
                "#", //
            ],
        );
    }

    #[test]
    fn vertical_2px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(2, 10)),
            &[
                "##", //
                "##", //
                "##", //
                "##", //
                "##", //
                "##", //
                "##", //
                "##", //
                "##", //
                "##", //
            ],
        );
    }

    #[test]
    fn vertical_3px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(3, 10)),
            &[
                " # ", //
                "###", //
                "###", //
                "###", //
                "###", //
                "###", //
                "###", //
                "###", //
                "###", //
                " # ", //
            ],
        );
    }

    #[test]
    fn vertical_4px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(4, 10)),
            &[
                " ## ", //
                "####", //
                "####", //
                "####", //
                "####", //
                "####", //
                "####", //
                "####", //
                "####", //
                " ## ", //
            ],
        );
    }

    #[test]
    fn vertical_5px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new(5, 10)),
            &[
                "  #  ", //
                " ### ", //
                "#####", //
                "#####", //
                "#####", //
                "#####", //
                "#####", //
                "#####", //
                " ### ", //
                "  #  ", //
            ],
        );
    }

    #[test]
    fn square_1px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new_equal(1)),
            &[
                "#", //
            ],
        );
    }

    #[test]
    fn square_2px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new_equal(2)),
            &[
                "##", //
                "##", //
            ],
        );
    }

    #[test]
    fn square_3px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new_equal(3)),
            &[
                " # ", //
                "###", //
                " # ", //
            ],
        );
    }

    #[test]
    fn square_4px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new_equal(4)),
            &[
                " ## ", //
                "####", //
                "####", //
                " ## ", //
            ],
        );
    }

    #[test]
    fn square_5px() {
        test_segment(
            Rectangle::new(Point::zero(), Size::new_equal(5)),
            &[
                "  #  ", //
                " ### ", //
                "#####", //
                " ### ", //
                "  #  ", //
            ],
        );
    }

    #[test]
    fn top_left_horizontal() {
        test_segment(
            Rectangle::new(Point::new(2, 3), Size::new(7, 3)),
            &[
                "         ", //
                "         ", //
                "         ", //
                "   ##### ", //
                "  #######", //
                "   ##### ", //
            ],
        );
    }

    #[test]
    fn top_left_vertical() {
        test_segment(
            Rectangle::new(Point::new(3, 1), Size::new(3, 6)),
            &[
                "      ", //
                "    # ", //
                "   ###", //
                "   ###", //
                "   ###", //
                "   ###", //
                "    # ", //
            ],
        );
    }
}
