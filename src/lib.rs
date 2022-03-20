//! `eg-seven-segment` is a seven-segment text renderer for use with
//! [`embedded-graphics`]. It can be used to draw seven-segment displays with
//! different sizes and styles.
//!
//! ![eg-seven-segment example][img1]
//!
//! # Examples
//!
//! The most convenient way to use this crate is by using the [`SevenSegmentStyle`] as a
//! character style for an [`embedded-graphics`] [`Text`]:
//!
//! ```
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{prelude::*, text::Text, pixelcolor::Rgb888};
//! use eg_seven_segment::SevenSegmentStyleBuilder;
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::new();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Define a new style.
//! let style = SevenSegmentStyleBuilder::new()
//!     .digit_size(Size::new(10, 20)) // digits are 10x20 pixels
//!     .digit_spacing(5)              // 5px spacing between digits
//!     .segment_width(5)              // 5px wide segments
//!     .segment_color(Rgb888::GREEN)  // active segments are green
//!     .build();
//!
//! // Use the style to draw text to a embedded-graphics `DrawTarget`.
//! Text::new("12:42", Point::new(5, 25), style).draw(&mut display)?;
//! # Ok(())
//! # }
//! ```
//!
//! Individual digits can also be drawn by using the [`Digit`] drawable:
//!
//! ```
//! # fn main() -> Result<(), core::convert::Infallible> {
//! use embedded_graphics::{prelude::*, text::Text, pixelcolor::Rgb888};
//! use eg_seven_segment::{SevenSegmentStyleBuilder, Digit, Segments};
//! # use embedded_graphics::mock_display::MockDisplay;
//! # let mut display = MockDisplay::new();
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! // Define a new style.
//! let style = SevenSegmentStyleBuilder::new()
//!     .digit_size(Size::new(10, 20)) // digits are 10x20 pixels
//!     .digit_spacing(5)              // 5px spacing between digits
//!     .segment_width(5)              // 5px wide segments
//!     .segment_color(Rgb888::GREEN)  // active segments are green
//!     .build();
//!
//! // Draw digit with active segment A at the origin.
//! let next = Digit::new(Segments::A, Point::zero())
//!     .into_styled(style)
//!     .draw(&mut display)?;
//!
//! // Draw `0` digit the the right of the previous digit.
//! Digit::new('0'.try_into().unwrap(), next)
//!     .into_styled(style)
//!     .draw(&mut display)?;
//! # Ok(())
//! # }
//! ```
//!
//! <!-- README-LINKS
//! [`embedded-graphics`](https://docs.rs/embedded-graphics)
//! [`Text`](https://docs.rs/embedded-graphics/latest/embeddded_graphics/text/struct.Text.html)
//! [`SevenSegmentStyle`](https://docs.rs/eg-seven-segment/latest/eg_seven_segment/struct.SevenSegmentStyle.html)
//! [`Digit`](https://docs.rs/eg-seven-segment/latest/eg_seven_segment/struct.Digit.html)
//! [img1]: assets/styles.png
//! README-LINKS -->
//!
//! [`embedded-graphics`]: embedded_graphics
//! [`Text`]: embedded_graphics::text::Text
#![doc = include_str!("../assets/styles.png_base64")]
//
#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(rustdoc::private_intra_doc_links)]

mod digit;
mod segment;
mod segments;
mod seven_segment_style;
mod seven_segment_style_builder;

pub use digit::Digit;
pub use segments::Segments;
pub use seven_segment_style::SevenSegmentStyle;
pub use seven_segment_style_builder::SevenSegmentStyleBuilder;
