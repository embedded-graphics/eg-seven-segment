# eg-seven-segment

`eg-seven-segment` is a seven-segment display text renderer for use with
[`embedded-graphics`]. The appearance of the drawn digits can be configured
to achieve a wide variety of styles.

![eg-seven-segment example][img1]

## Examples

The most convenient way to use this crate is by using the [`SevenSegmentStyle`] as a
character style for an [`embedded-graphics`] [`Text`]:

```rust
use embedded_graphics::{prelude::*, text::Text, pixelcolor::Rgb888};
use eg_seven_segment::SevenSegmentStyleBuilder;

// Define a new style.
let style = SevenSegmentStyleBuilder::new()
    .digit_size(Size::new(10, 20)) // digits are 10x20 pixels
    .digit_spacing(5)              // 5px spacing between digits
    .segment_width(5)              // 5px wide segments
    .segment_color(Rgb888::GREEN)  // active segments are green
    .build();

// Use the style to draw text to a embedded-graphics `DrawTarget`.
Text::new("12:42", Point::new(5, 25), style).draw(&mut display)?;
```

Individual digits can also be drawn by using the [`Digit`] drawable:

```rust
use embedded_graphics::{prelude::*, text::Text, pixelcolor::Rgb888};
use eg_seven_segment::{SevenSegmentStyleBuilder, Digit, Segments};

// Define a new style.
let style = SevenSegmentStyleBuilder::new()
    .digit_size(Size::new(10, 20)) // digits are 10x20 pixels
    .digit_spacing(5)              // 5px spacing between digits
    .segment_width(5)              // 5px wide segments
    .segment_color(Rgb888::GREEN)  // active segments are green
    .build();

// Draw digit with active segment A at the origin.
let next = Digit::new(Segments::A, Point::zero())
    .into_styled(style)
    .draw(&mut display)?;

// Draw `0` digit the the right of the previous digit.
Digit::new('0'.try_into().unwrap(), next)
    .into_styled(style)
    .draw(&mut display)?;
```

[`embedded-graphics`]: https://docs.rs/embedded-graphics
[`Text`]: https://docs.rs/embedded-graphics/latest/embeddded_graphics/text/struct.Text.html
[`SevenSegmentStyle`]: https://docs.rs/eg-seven-segment/latest/eg_seven_segment/struct.SevenSegmentStyle.html
[`Digit`]: https://docs.rs/eg-seven-segment/latest/eg_seven_segment/struct.Digit.html
[img1]: assets/styles.png

[`embedded-graphics`]: embedded_graphics
[`Text`]: embedded_graphics::text::Text

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
