use crate::SevenSegmentStyle;
use embedded_graphics::prelude::*;

/// Seven-segment character style builder.
#[derive(Debug)]
pub struct SevenSegmentStyleBuilder<C> {
    style: SevenSegmentStyle<C>,
}

impl<C: PixelColor> SevenSegmentStyleBuilder<C> {
    /// Creates a new builder.
    pub fn new() -> Self {
        // TODO: set better default values
        // TODO: add default values to docs
        Self {
            style: SevenSegmentStyle {
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

    /// Resets the segment color to transparent.
    pub fn reset_segment_color(mut self) -> Self {
        self.style.segment_color = None;

        self
    }

    /// Sets the inactive segment color.
    pub fn inactive_segment_color(mut self, inactive_segment_color: C) -> Self {
        self.style.inactive_segment_color = Some(inactive_segment_color);

        self
    }

    /// Resets the inactive segment color to transparent.
    pub fn reset_inactive_segment_color(mut self) -> Self {
        self.style.inactive_segment_color = None;

        self
    }

    /// Builds the text style.
    pub fn build(self) -> SevenSegmentStyle<C> {
        self.style
    }
}

impl<C: PixelColor> Default for SevenSegmentStyleBuilder<C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: PixelColor> From<&SevenSegmentStyle<C>> for SevenSegmentStyleBuilder<C> {
    fn from(style: &SevenSegmentStyle<C>) -> Self {
        Self { style: *style }
    }
}
