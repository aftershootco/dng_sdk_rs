/// The maximum number of previews (in addition to the main IFD's thumbnail)
/// that we support embedded in a DNG.

pub const MAX_DNGPREVIEWS: usize = 20;

/// Maximum supported number of semantic masks.

pub const MAX_SEMANTIC_MASKS: usize = 100;

/// The maximum number of SubIFDs that will be parsed.

pub const MAX_SUB_IFDS: usize = MAX_DNGPREVIEWS + MAX_SEMANTIC_MASKS + 1;

/// The maximum number of chained IFDs that will be parsed.

pub const MAX_CHAINED_IFDS: usize = 10;

/// The maximum number of samples per pixel.  (CMYK + transparency needs 5)

pub const MAX_SAMPLES_PER_PIXEL: usize = 5;

/// Maximum number of color planes.

pub const MAX_COLOR_PLANES: usize = 4;

/// The maximum size of a CFA repeating pattern.

pub const MAX_CFA_PATTERN: usize = 8;

/// The maximum size of a black level repeating pattern.

pub const MAX_BLACK_PATTERN: usize = 8;

/// The maximum number of masked area rectangles.

pub const MAX_MASKED_AREAS: usize = 4;

/// The maximum image size supported (pixels per side).

#[cfg(feature = "dng_big_image")]
// #if qDNGBigImage
pub const MAX_IMAGE_SIDE: usize = 300000;
#[cfg(not(feature = "dng_big_image"))]
pub const MAX_IMAGE_SIDE: usize = 65000;

/// The maximum number of tone curve points supported.

pub const MAX_TONE_CURVE_POINTS: usize = 8192;

/// Maximum number of MP threads for dng_area_task operations.

#[cfg(feature = "dng64bit")]
pub const MAX_MPTHREADS: usize = 128; // EP! Needs much larger max!
#[cfg(not(feature = "dng64bit"))]
pub const MAX_MPTHREADS: usize = 8;

/// Maximum supported value of Stage3BlackLevelNormalized.

pub const MAX_STAGE3_BLACLEVEL_NORMALIZED: f64 = 0.2;

/// Maximum supported number of points in a ProfileGainTableMap. Currently set
/// to 64 megabytes.

pub const MAX_PROFILE_GAIN_TABLE_MAP_POINTS: usize = 16777216;

/// Minimum and maximum gain values in a ProfileGainTableMap. The
/// specification only requires that values be positive, but this SDK
/// implementation assumes that values outside the following range are errors.

pub const PROFILE_GAIN_TABLE_MAP_MIN_GAIN_VALUE: f64 = 0.000244140625; // 1 / 4096
pub const PROFILE_GAIN_TABLE_MAP_MAX_GAIN_VALUE: f32 = 4096.0_f32;

pub const MIN_SPECTRUM_SAMPLES: usize = 2;

/// The maximum number of spectral power samples for an illuminant.
/// A sampling that covers 360 to 730 nm in 1 nm steps is just 371 samples,
/// so 1000 seems more than enough.

pub const MAX_SPECTRUM_SAMPLES: usize = 1000;
