use eg_seven_segment::SevenSegmentTextStyleBuilder;
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, text::Text};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

fn main() {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(512, 256));

    let small_green = SevenSegmentTextStyleBuilder::new()
        .digit_size(Size::new(24, 48))
        .digit_spacing(10)
        .segment_width(6)
        .segment_color(Rgb888::GREEN)
        .build();

    let tiny_blue = SevenSegmentTextStyleBuilder::new()
        .digit_size(Size::new(16, 24))
        .digit_spacing(10)
        .segment_width(2)
        .segment_color(Rgb888::BLUE)
        .build();

    let large_red = SevenSegmentTextStyleBuilder::new()
        .digit_size(Size::new(60, 100))
        .digit_spacing(10)
        .segment_width(16)
        .segment_color(Rgb888::RED)
        .inactive_segment_color(Rgb888::new(0x30, 0x00, 0x00))
        .build();

    Text::new("-----\n12:42\n13°C\n-----", Point::new(20, 60))
        .into_styled(small_green)
        .draw(&mut display)
        .unwrap();

    Text::new("3.141", Point::new(200, 200))
        .into_styled(large_red)
        .draw(&mut display)
        .unwrap();

    Text::new("hello rust", Point::new(200, 60))
        .into_styled(tiny_blue)
        .draw(&mut display)
        .unwrap();

    let mut window = Window::new("Styles", &OutputSettings::default());
    window.show_static(&display);
}
