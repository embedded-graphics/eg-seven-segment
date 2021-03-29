//! `eg-seven-segment`is a seven-segment text renderer for use with
//! [`embedded-graphics`]. It can be used to draw seven-segment displays with
//! any size.
//!
//! ![eg-seven-segment example][img1]
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
//! [img1]: data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAgAAAAEABAMAAAA3vtNUAAAAD1BMVEUAAAAAAP8A/wAwAAD/AAAy4AU/AAAEl0lEQVR42u2dbZLiIBBArYkX8AZTnmCqcoH8yP3PtOu4ukL4aLCbCHnvz1YyvaiPTkRo8XQCAAAAAAAAAAAAAAAAAAAAgCNx/UX5REd83Z/8t+qJ/hLgpfs0TiDA5/KLyQt4t+k2l8AHC7C6CV4qDqpev2prmh2DAAQgAAEIQAACDivAHaEJh2vxMOFfDEecCKhLzUvF9XCRJXrg3AfeAxCAAAQgAAEIQMABBUiHa28PGD92KIwAAAAAAOgNeblLz7VRceQVT13XRuW6VdKxXddGIWDzavzEvsYCspGdXvnfbsd+edf28zgb2W3ip088j7ORCEAAAhCAAAQMI8D/FwEIQAACEMDbIAKGEyCZD9j8YaT5AMGM0OYPQ80IORkdnukb6wujCIhdAtfobL+f8deh1gUed7LnvS6aG5LIjt8G8y9IHomAIQRkx8gISLM65ONnB8OQRgLOroAlFz+5T/3HLKSVgHUtS4F5znaeTkgjAWdfwFKUAKHO0wlpJWBdy1JgnrOdpxPS6G3wvBWwFCXAtvN0QloJWNeyFJjnbOfphDQScA4JWIoSwO88nRAEtBEw/YQEnJd4fOipO/HnJRsiaKWVgL8PG0iA+F1gnkO3Lyd+XbMhglYaCZh+H3eTANGL4BYf6LrX+NtBJkTQSvV8QGHkvwfeJECsM27xU6DrXuJvB5kQQSu1M0KFkdPzkb0EiHTGPT7Qdf/j7wfJEEEr1XOChZEvD+0lQLgz7vFToOue8feDZIiglUYCJuexnQQIdsYjPtB1j/jHQSJE0IpgdlClWDo8BIsPBwXx4f/snC191ETHvhvZsQAdEIAABCAAAQhAAAIQMMBQuGo3/X4FKO2m368Apd30EdCbgGj137WuOKo3AdH6z02FqLA4qjcB+UXgwsoQBCAAAQhAAAIQcFwBhktjiSbrl8bUBdgtjiabrF4c1RdgtjyebLJ6eVxfgFWBRKbJ2gIJAwFGJTKZJmtLZAwE2BRJZZt8s0hKU4BJmVy2ycoyuaL5AOnMsEGhpKDJukLJkhmh05AC3pwCbHQJCJqsvAQsBBjcBAVNVt4EE1+e9udAxfdA/bdBQZPvVoqeAl+KdxNAfBM0GAgJmlQfCOUj2w2FBU3qD4WrBRh8GBI0qf9hqFaAwcdhQZMGH4drBRhMiOiEIAABCEAAAhCAAAQgAAEIkM4HbCJ3KpIyFVDwswm7lcnZCiiYAtyrUBIBbQTkE3zwS0BQDD/kTVAbBCAAAQhAAAIQcFwBgnUs53kVLI0Zb6KimwKplUy3Y8SLo9bb6KimQHIt2xUgXh633khJNQWS1QzepSkskLDaSsskBdL1LJ4AYYmM0WZqNimQrmjyb86iIimr7fRMUiBT0+YLEJXJWW2oaEKmqnHz9rznlpoIMCG9re72zI7b6jZLgSUlYMeNlZulQHqIvt/W2q1SYEkL2G9z9VYpkPuQttv2+o1SYMkJ2O0HFhqlQP5j+l4/sQEAAHBIKnbPq9pf71Op2D2vbn+9z06Aos3jxvrtzcMLOPwlcPibIAAAAAAAAAAAAAAAAAAAAAAAAAAADMQfZ4QGBaIyp8YAAAAASUVORK5CYII=

#![no_std]
#![deny(missing_docs)]

mod segment;
mod segments;
mod seven_segment_text_style;
mod seven_segment_text_style_builder;

pub use segments::Segments;
pub use seven_segment_text_style::SevenSegmentTextStyle;
pub use seven_segment_text_style_builder::SevenSegmentTextStyleBuilder;
