use crate::SevenSegmentTextStyle;
use embedded_graphics::prelude::*;

pub struct SevenSegmentTextStyleBuilder<C> {
    style: SevenSegmentTextStyle<C>,
}

impl<C: PixelColor> SevenSegmentTextStyleBuilder<C> {
    pub fn new() -> Self {
        // TODO: set better default values
        // TODO: add default values to docs
        Self {
            style: SevenSegmentTextStyle {
                digit_size: Size::new(12, 24),
                digit_spacing: 5,
                segment_width: 3,
                segment_color: None,
                inactive_segment_color: None,
            },
        }
    }

    /// Sets the digit size.
    pub fn digit_size(mut self, digit_size: Size) -> Self {
        self.style.digit_size = digit_size;

        self
    }

    /// Sets the digit spacing.
    pub fn digit_spacing(mut self, digit_spacing: u32) -> Self {
        self.style.digit_spacing = digit_spacing;

        self
    }

    /// Sets the segment width.
    pub fn segment_width(mut self, segment_width: u32) -> Self {
        self.style.segment_width = segment_width;

        self
    }

    /// Sets the segment color.
    pub fn segment_color(mut self, segment_color: C) -> Self {
        self.style.segment_color = Some(segment_color);

        self
    }

    /// Sets the inactive segment color.
    pub fn inactive_segment_color(mut self, inactive_segment_color: C) -> Self {
        self.style.inactive_segment_color = Some(inactive_segment_color);

        self
    }

    /// Builds the text style.
    pub fn build(self) -> SevenSegmentTextStyle<C> {
        self.style
    }
}

impl<C: PixelColor> From<&SevenSegmentTextStyle<C>> for SevenSegmentTextStyleBuilder<C> {
    fn from(style: &SevenSegmentTextStyle<C>) -> Self {
        Self { style: *style }
    }
}
