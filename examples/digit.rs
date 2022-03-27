//! This example uses the `Digit` drawable to draw a spinning throbber.
//!
//! The spinning progress indicator animation requires custom characters, which
//! are defined using the `Segments` bitfield.

use std::{iter, time::Duration};

use eg_seven_segment::{Digit, Segments, SevenSegmentStyleBuilder};
use embedded_graphics::{pixelcolor::Rgb888, prelude::*, text::renderer::TextRenderer};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, SimulatorEvent, Window};

fn main() -> Result<(), std::convert::Infallible> {
    let mut display = SimulatorDisplay::<Rgb888>::new(Size::new(256, 256));
    let mut window = Window::new("Digit drawable", &OutputSettings::default());

    let style = SevenSegmentStyleBuilder::new()
        .digit_size(Size::new(24, 48))
        .digit_spacing(6)
        .segment_width(6)
        .segment_color(Rgb888::RED)
        .inactive_segment_color(Rgb888::new(0x30, 0x00, 0x00))
        .build();

    // Create frame iterators for different types of progress indicators.
    // The iterators are infinitely repeating and return animation frames of type `Segments`.
    let small_top = Throbber::SmallTop.frames();
    let small_bottom = Throbber::SmallBottom.frames();
    let medium = Throbber::Medium.frames();
    let large_1 = Throbber::Large1.frames();
    let large_2 = Throbber::Large2.frames();

    let throbbers = &mut [
        &mut [small_top] as &mut [_],
        &mut [small_bottom],
        &mut [medium],
        &mut [large_1, large_2],
    ];

    let start_position = Point::new(100, 25);

    'main: loop {
        let mut position = start_position;

        for line in throbbers.iter_mut() {
            for throbber in line.iter_mut() {
                // Get active segments for next animation frame.
                let segments = throbber.next().unwrap();

                // Draw the digit at `position`.
                // The returned `Point` is the position of the next digit in the same line.
                position = Digit::new(segments, position)
                    .into_styled(style)
                    .draw(&mut display)?;
            }

            position = Point::new(start_position.x, position.y) + Size::new(0, style.line_height());
        }

        window.update(&display);

        for event in window.events() {
            if let SimulatorEvent::Quit = event {
                break 'main;
            }
        }

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

pub enum Throbber {
    SmallTop,
    SmallBottom,
    Medium,
    Large1,
    Large2,
}

impl Throbber {
    fn frames(self) -> impl Iterator<Item = Segments> {
        const NONE: Segments = Segments::empty();

        let frames: &[_] = match self {
            Throbber::SmallTop => &[Segments::A, Segments::B, Segments::G, Segments::F],
            Throbber::SmallBottom => &[Segments::C, Segments::D, Segments::E, Segments::G],
            Throbber::Medium => &[
                Segments::A,
                Segments::B,
                Segments::C,
                Segments::D,
                Segments::E,
                Segments::F,
            ],
            Throbber::Large1 => &[
                Segments::A,
                NONE,
                NONE,
                NONE,
                NONE,
                Segments::D,
                Segments::E,
                Segments::F,
            ],
            Throbber::Large2 => &[
                NONE,
                Segments::A,
                Segments::B,
                Segments::C,
                Segments::D,
                NONE,
                NONE,
                NONE,
            ],
        };

        iter::repeat(frames.iter().copied()).flatten()
    }
}
