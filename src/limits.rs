/// The maximum number of previews (in addition to the main IFD's thumbnail)
/// that we support embedded in a DNG.

pub const K_MAX_DNGPREVIEWS: u32 = 20;

/// Maximum supported number of semantic masks.

pub const K_MAX_SEMANTIC_MASKS: u32 = 100;

/// The maximum number of SubIFDs that will be parsed.

pub const K_MAX_SUB_IFDS: u32 = K_MAX_DNGPREVIEWS + K_MAX_SEMANTIC_MASKS + 1;

/// The maximum number of chained IFDs that will be parsed.

pub const K_MAX_CHAINED_IFDS: u32 = 10;

/// The maximum number of samples per pixel.  (CMYK + transparency needs 5)

pub const K_MAX_SAMPLES_PER_PIXEL: u32 = 5;

/// Maximum number of color planes.

pub const K_MAX_COLOR_PLANES: usize = 4;

/// The maximum size of a CFA repeating pattern.

pub const K_MAX_CFAPATTERN: u32 = 8;

/// The maximum size of a black level repeating pattern.

pub const K_MAX_BLACK_PATTERN: u32 = 8;

/// The maximum number of masked area rectangles.

pub const K_MAX_MASKED_AREAS: u32 = 4;

/// The maximum image size supported (pixels per side).

#[cfg(feature = "dng_big_image")]
// #if qDNGBigImage
pub const K_MAX_IMAGE_SIDE: u32 = 300000;
#[cfg(not(feature = "dng_big_image"))]
pub const K_MAX_IMAGE_SIDE: u32 = 65000;

/// The maximum number of tone curve points supported.

pub const K_MAX_TONE_CURVE_POINTS: u32 = 8192;

/// Maximum number of MP threads for dng_area_task operations.

#[cfg(feature = "dng64bit")]
pub const K_MAX_MPTHREADS: u32 = 128; // EP! Needs much larger max!
#[cfg(not(feature = "dng64bit"))]
pub const K_MAX_MPTHREADS: u32 = 8;

/// Maximum supported value of Stage3BlackLevelNormalized.

pub const K_MAX_STAGE3_BLACK_LEVEL_NORMALIZED: f64 = 0.2;

/// Maximum supported number of points in a ProfileGainTableMap. Currently set
/// to 64 megabytes.

pub const K_MAX_PROFILE_GAIN_TABLE_MAP_POINTS: u32 = 16777216;

/// Minimum and maximum gain values in a ProfileGainTableMap. The
/// specification only requires that values be positive, but this SDK
/// implementation assumes that values outside the following range are errors.

pub const K_PROFILE_GAIN_TABLE_MAP_MIN_GAIN_VALUE: f32 = 0.000244140625_f32; // 1 / 4096
pub const K_PROFILE_GAIN_TABLE_MAP_MAX_GAIN_VALUE: f32 = 4096.0_f32;

pub const K_MIN_SPECTRUM_SAMPLES: u32 = 2;

/// The maximum number of spectral power samples for an illuminant.
/// A sampling that covers 360 to 730 nm in 1 nm steps is just 371 samples,
/// so 1000 seems more than enough.

pub const K_MAX_SPECTRUM_SAMPLES: u32 = 1000;
