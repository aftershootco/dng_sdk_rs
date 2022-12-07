use crate::{host::Host, ifd::Ifd};

pub struct DngReader {}
pub struct DngReadTiles {}

impl DngReader {
    pub const IMAGE_BUFFER_SIZE:u32 =128 * 1024;

    pub fn read(host: &Host, ifd: &Ifd) -> Self {
        todo!()
    }
}

