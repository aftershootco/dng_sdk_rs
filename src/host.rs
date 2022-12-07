pub struct Host {
    /// Does the host require all the image metadata (vs. just checking
    /// to see if the file is readable)?

    /// Whether all XMP metadata should be parsed.
    /// Defaults to true. One might not want metadata when doing a quick check
    /// to see if a file is readable.
    needs_metadata: bool,
    /// Does the host require actual image data (vs. just getting metadata
    /// or just checking to see if the file is readable)?
    needs_image: bool,

    /// If we need the image data, can it be read at preview quality?
    for_preview: bool,

    /// If non-zero, the minimum size (longer of the two pixel dimensions)
    /// image to read.  If zero, or if the full size image is smaller than
    /// this, read the full size image.
    minimum_size: u32,

    /// What is the preferred size for a preview image?	This can
    /// be slightly larger than the minimum size.  Zero if we want
    /// the full resolution image.
    preffered_size: u32,

    /// What is the maximum size for a preview image?  Zero if there
    /// is no maximum size limit.
    maximum_size: u32,

    /// The fraction of the image kept after a crop.	 This is used to
    /// adjust the sizes to take into account the cropping that
    /// will be peformed.
    crop_factor: f64,

    /// What DNG version should we keep enough data to save?
    save_dng_version: u32,

    /// Do we want to force saving to a linear DNG?
    save_linear_dng: bool,

    /// Keep the original raw file data block?
    keep_original_file: bool,

    /// Should we ignore the enhanced IFD when reading DNGs?
    ignore_enhanced_ifd: bool,

    /// Is this host being used to perform a negative read for fast
    /// conversion to DNG?
    for_fast_save_to_dng: bool,

    fast_save_to_dng_size: u32,

    preserve_stage2: bool,
}
