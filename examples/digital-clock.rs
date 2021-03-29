use eg_seven_segment::SevenSegmentTextStyleBuilder;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let character_style = SevenSegmentTextStyleBuilder::new()
        .segment_color(BinaryColor::On)
        .build();

    let text_style = TextStyleBuilder::new()
        .character_style(character_style)
        .alignment(Alignment::Center)
        .baseline(Baseline::Middle)
        .build();

    Text::new("12:42", display.bounding_box().center())
        .into_styled(text_style)
        .draw(&mut display)
        .unwrap();

    let settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let mut window = Window::new("Digital clock", &settings);
    window.show_static(&display);
}
