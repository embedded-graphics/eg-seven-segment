use embedded_graphics::{
    geometry::AnchorPoint,
    prelude::*,
    primitives::{Rectangle, Styled, StyledDrawable},
};

use crate::{segment::Segment, Segments, SevenSegmentStyle};

/// Single digit drawable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digit {
    segments: Segments,
    position: Point,
}

impl Digit {
    /// Creates a new digit.
    pub fn new(segments: Segments, position: Point) -> Self {
        Self { segments, position }
    }

    /// Applies a style to this digit.
    pub fn into_styled<C: PixelColor>(
        self,
        style: SevenSegmentStyle<C>,
    ) -> Styled<Self, SevenSegmentStyle<C>> {
        Styled {
            primitive: self,
            style,
        }
    }
}

impl<C: PixelColor> StyledDrawable<SevenSegmentStyle<C>> for Digit {
    type Color = C;
    type Output = Point;

    fn draw_styled<D>(
        &self,
        style: &SevenSegmentStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        let rect = Rectangle::new(self.position, style.digit_size);

        let vertical_size = Size::new(style.digit_size.width, style.segment_width);
        let horizontal_size_top = Size::new(
            style.segment_width,
            (style.digit_size.height + style.segment_width) / 2,
        );
        let horizontal_size_bottom = Size::new(
            style.segment_width,
            (style.digit_size.height + style.segment_width + 1) / 2,
        );

        if let Some(color) = style.state_color(self.segments.contains(Segments::A)) {
            Segment::with_reduced_size(rect.resized(vertical_size, AnchorPoint::TopLeft), color)
                .draw(target)?;
        }

        if let Some(color) = style.state_color(self.segments.contains(Segments::B)) {
            Segment::with_reduced_size(
                rect.resized(horizontal_size_top, AnchorPoint::TopRight),
                color,
            )
            .draw(target)?;
        }

        if let Some(color) = style.state_color(self.segments.contains(Segments::C)) {
            Segment::with_reduced_size(
                rect.resized(horizontal_size_bottom, AnchorPoint::BottomRight),
                color,
            )
            .draw(target)?;
        }

        if let Some(color) = style.state_color(self.segments.contains(Segments::D)) {
            Segment::with_reduced_size(rect.resized(vertical_size, AnchorPoint::BottomLeft), color)
                .draw(target)?;
        }

        if let Some(color) = style.state_color(self.segments.contains(Segments::E)) {
            Segment::with_reduced_size(
                rect.resized(horizontal_size_bottom, AnchorPoint::BottomLeft),
                color,
            )
            .draw(target)?;
        }

        if let Some(color) = style.state_color(self.segments.contains(Segments::F)) {
            Segment::with_reduced_size(
                rect.resized(horizontal_size_top, AnchorPoint::TopLeft),
                color,
            )
            .draw(target)?;
        }

        if let Some(color) = style.state_color(self.segments.contains(Segments::G)) {
            Segment::with_reduced_size(rect.resized(vertical_size, AnchorPoint::CenterLeft), color)
                .draw(target)?;
        }

        Ok(self.position + style.digit_size.x_axis() + Size::new(style.digit_spacing, 0))
    }
}
