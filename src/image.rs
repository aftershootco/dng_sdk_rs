use crate::rect::Rect;

pub enum EdgeOption {
    /// Leave edge pixels unchanged
    None,
    /// Pad with zeroes
    Zero,
    /// Repeat edge pixels
    Repeat,
    /// Repeat edge pixels, except for last plane which is zero padded.
    EdgeRepeatZeroLast,

    /// Wrap edge pixels horizontally, repeat edge pixels vertically.
    EdgeWrapHorizontal,

    /// Wrap edge pixels vertically, repeat edge pixels horizontally.
    EdgeWrapVertical,

    /// Wrap edge pixels in all directions (horizontal, vertical, diagonal).
    EdgeWrapAll,
}

#[repr(u32)]
pub enum PixelType {
    Byte,
    Short,
    SShort,
    Long,
    Float,
}

pub struct Image {
    /// The bounds for this image
    bounds: Rect<i32>,
    /// The number of image planes.
    planes: u32,
    /// Basic Pixel type (TIFF tag type code)
    pixel_type: PixelType,
    /// How to handle requests to get image areas outside the image bounds.
    edge_option: EdgeOption,
}

pub trait ImageTrait<SubPixel> {
    /// Shrink the bounds of the image to the given rectangle
    fn trim<T>(&mut self, bounds: Rect<T>) -> &mut Self;
    fn roatate(&mut self) -> &mut Self;
}
