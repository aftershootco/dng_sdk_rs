use crate::image::PixelType;
use crate::rect::Rect;

pub struct PixelBuffer<'pb> {
    /// The area this buffer holds
    pub area: Rect<i32>,
    /// The range of planes this buffer holds
    pub plane: u32,
    pub planes: u32,
    /// Steps between pixels
    pub row_step: u64,
    pub col_step: u64,
    /// Steps between planes
    pub plane_step: u64,

    /// The pixel type (TIFF tag type code)
    pub pixel_type: PixelType,
    /// The size of the pixel type in bytes
    pub pixel_size: u32,
    // NOTE: Since this is ported from C++ I'll have to think of the structure a bit more
    // We can use Arc<Mutext<T>>/Arc<RwLock<T> but I think performance will suffer with that
    // We can use a RefCell/Cell<T> but that's not thread safe
    /// The pixel data
    pub data: &'pb [u8],
    /// Do we have write access to this data ?
    pub dirty: bool,
}

impl<'pb> PixelBuffer<'pb> {
    #[inline]
    pub fn internal_pixel(&self, row: u64, col: u64, plane: u64) -> &'pb [u8] {
        // TO DO: review this. do we set up buffers sometimes with "col" parameter
        // equal to 0, which would then cause this exception to throw?!
        let offset = (self.pixel_size as u64
            * (self.row_step * (row - self.area.top as u64)
                + self.col_step * (col - self.area.left as u64)
                + self.plane_step * (plane - self.plane as u64))) as usize;
        &self.data[offset..offset + self.pixel_size as usize]
    }
}
