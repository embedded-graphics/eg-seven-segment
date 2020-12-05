use crate::Segments;
use core::convert::TryFrom;
use embedded_graphics::{prelude::*, primitives::Rectangle, style::TextStyle};

#[non_exhaustive]
pub struct SevenSegmentTextStyle<C> {
    pub digit_size: Size,
    pub digit_spacing: u32,
    pub segment_width: u32,
    pub segment_color: Option<C>,
    pub inactive_segment_color: Option<C>,
}

impl<C: PixelColor> SevenSegmentTextStyle<C> {
    fn draw_segment<D>(
        &self,
        rectangle: &Rectangle,
        active: bool,
        target: &mut D,
    ) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        let color = if active {
            self.segment_color
        } else {
            self.inactive_segment_color
        };

        let color = if let Some(color) = color {
            color
        } else {
            return Ok(());
        };

        let horizontal = rectangle.size.width > rectangle.size.height;
        let major_size = if horizontal {
            rectangle.size.height
        } else {
            rectangle.size.width
        };
        let offset = major_size / 2 + 1 + (major_size - 1) / 2;

        let mut rect = if horizontal {
            Rectangle::new(
                rectangle.top_left + Size::new(offset, 0),
                Size::new(rectangle.size.width - offset * 2, 1),
            )
        } else {
            Rectangle::new(
                rectangle.top_left + Size::new(0, offset),
                Size::new(1, rectangle.size.height - offset * 2),
            )
        };

        for _ in 0..(major_size + 1) / 2 {
            target.fill_solid(&rect, color)?;

            if horizontal {
                rect.top_left += Point::new(-1, 1);
                rect.size.width += 2;
            } else {
                rect.top_left += Point::new(1, -1);
                rect.size.height += 2;
            }
        }

        let delta = if major_size % 2 == 0 { 1 } else { 2 };
        if horizontal {
            rect.top_left.x += delta;
            rect.size.width -= delta as u32 * 2;
        } else {
            rect.top_left.y += delta;
            rect.size.height -= delta as u32 * 2;
        }

        for _ in 0..major_size / 2 {
            target.fill_solid(&rect, color)?;

            rect.top_left += Point::new(1, 1);
            if horizontal {
                rect.size.width -= 2;
            } else {
                rect.size.height -= 2;
            }
        }

        Ok(())
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
                let dx = self.digit_size.width - self.segment_width;

                let mut rect = Rectangle::new(
                    position,
                    Size::new(self.digit_size.width, self.segment_width),
                );
                self.draw_segment(&rect, segments.contains(Segments::A), target)?;

                rect.top_left += Size::new(0, (self.digit_size.height - self.segment_width) / 2);
                self.draw_segment(&rect, segments.contains(Segments::G), target)?;

                rect.top_left +=
                    Size::new(0, (self.digit_size.height - self.segment_width + 1) / 2);
                self.draw_segment(&rect, segments.contains(Segments::D), target)?;

                rect = Rectangle::new(
                    position,
                    Size::new(
                        self.segment_width,
                        self.digit_size.height / 2 + self.segment_width / 2,
                    ),
                );
                self.draw_segment(&rect, segments.contains(Segments::F), target)?;

                rect.top_left.x += dx as i32;
                self.draw_segment(&rect, segments.contains(Segments::B), target)?;

                rect = Rectangle::new(
                    position
                        + Size::new(0, self.digit_size.height / 2 - (self.segment_width + 1) / 2),
                    Size::new(
                        self.segment_width,
                        self.digit_size.height / 2 + (self.segment_width + 1) / 2,
                    ),
                );
                self.draw_segment(&rect, segments.contains(Segments::E), target)?;

                rect.top_left.x += dx as i32;
                self.draw_segment(&rect, segments.contains(Segments::C), target)?;

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
