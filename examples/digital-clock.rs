use eg_seven_segment::SevenSegmentTextStyleBuilder;
use embedded_graphics::{pixelcolor::BinaryColor, prelude::*, text::Text};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(128, 64));

    let style = SevenSegmentTextStyleBuilder::new()
        .segment_color(BinaryColor::On)
        .build();

    Text::new(
        "12:42",
        Point::new(10, 10)
        // TODO: use display center point when text alignment is implemented
        // display.bounding_box().anchor_point(AnchorPoint::Center),
    )
    .into_styled(style)
    .draw(&mut display)
    .unwrap();

    let settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let mut window = Window::new("Digital clock", &settings);
    window.show_static(&display);
}
