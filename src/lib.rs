//! `eg-seven-segment`is a seven-segment text renderer for use with
//! [`embedded-graphics`]. It can be used to draw seven-segment displays with
//! any size.
//!
//! # Examples
//!
//! ```
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{prelude::*, text::Text, pixelcolor::Rgb888};
//! use eg_seven_segment::SevenSegmentTextStyleBuilder;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::new();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Define a new style.
//! let style = SevenSegmentTextStyleBuilder::new()
//!     .digit_size(Size::new(10, 20)) // digits are 10x20 pixels
//!     .digit_spacing(5)              // 5px spacing between digits
//!     .segment_width(5)              // 5px wide segments
//!     .segment_color(Rgb888::GREEN)  // active segments are green
//!     .build();
//!
//! // Use the style to draw text to a embedded-graphics `DrawTarget`.
//! Text::new("12:42", Point::new(5, 25))
//!     .into_styled(style)
//!     .draw(&mut display)?;
//! # Ok(())
//! # }
//! ```
//!
//! [`embedded-graphics`]:  https://github.com/embedded-graphics/embedded-graphics

#![no_std]
#![deny(missing_docs)]

mod segment;
mod segments;
mod seven_segment_text_style;
mod seven_segment_text_style_builder;

pub use segments::Segments;
pub use seven_segment_text_style::SevenSegmentTextStyle;
pub use seven_segment_text_style_builder::SevenSegmentTextStyleBuilder;
