use crate::exif::Exif;
use crate::host::Host;
use crate::ifd::Ifd;
use crate::tag::values::Magic;

pub struct Shared;
pub struct Stream;

pub struct DngInfo<'info> {
    pub tiff_block_offset: u64,
    pub tiff_block_original_offset: u64,
    pub big_endian: bool,
    pub magic: Magic,
    pub exif: Exif<'info>,
    pub shared: Shared,
    pub main_index: i32,
    pub mask_index: i32,
    pub depth_index: i32,
    pub enhanced_index: i32,
    pub semantic_mask_indices: Vec<u32>,
    pub ifd: Vec<Ifd>,
    pub chained_ifd: Vec<Ifd>,
    pub chained_sub_ifd: Vec<Vec<Ifd>>,
    pub maker_note_next_ifd: u32,
}

impl DngInfo<'_> {
    pub fn parse_tag(
        host: Host,
        stream: Stream,
        exif: Exif,
        shared: &Shared,
        ifd: &Ifd,
        parent_code: u32,
        tag_code: u32,
        tag_type: u32,
        tag_count: u32,
        offset: u64,
        offset_delta: i64,
    ) {
        // let is_sub_ifd = parent_code >= tc_
    }
}
