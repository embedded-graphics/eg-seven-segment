//! This example demonstrates how custom seven segment digits can be included in strings,
//! which are displayed by using `Text`.

use eg_seven_segment::{Segments, SevenSegmentStyleBuilder};
use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

fn main() -> Result<(), std::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(512, 256));

    let character_style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(24, 48))
        .digit_spacing(10)
        .segment_width(6)
        .segment_color(Rgb888::GREEN)
        .build();

    let text_style = TextStyleBuilder::new()
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();

    // `Segments` values are be converted into `char`s to include them in a string.
    let text = format!(
        "{}{}{} 123 {}{}{}",
        char::from(Segments::D),
        char::from(Segments::D | Segments::G),
        char::from(Segments::A | Segments::D | Segments::G),
        char::from(Segments::A | Segments::D | Segments::G),
        char::from(Segments::D | Segments::G),
        char::from(Segments::D),
    );

    // Draw the text in the center of the display.
    Text::with_text_style(
        &text,
        display.bounding_box().center(),
        character_style,
        text_style,
    )
    .draw(&mut display)?;

    let mut window = Window::new("Custom digits", &OutputSettings::default());
    window.show_static(&display);

    Ok(())
}
