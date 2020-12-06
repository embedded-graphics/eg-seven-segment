#![no_std]

mod segment;
mod segments;
mod seven_segment_text_style;
mod seven_segment_text_style_builder;

pub use segments::Segments;
pub use seven_segment_text_style::SevenSegmentTextStyle;
pub use seven_segment_text_style_builder::SevenSegmentTextStyleBuilder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
