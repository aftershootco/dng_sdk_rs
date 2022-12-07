use crate::matrix::Vector;
use crate::point::Point;
use crate::traits::LinearFn;
use crate::types::non_zero_float::{NonZeroUF64, UF64};
use crate::types::rational::URational;
pub enum RawImageStage {
    RawImageStagePreOpcode1,
    RawImageStagePostOpcode1,
    RawImageStagePostOpcode2,
    RawImageStagePreOpcode3,
    RawImageStagePostOpcode3,
    RawImageStageNone,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseProfile {
    noise_functions: Vector<NoiseFunction, 3>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseFunction {
    scale: NonZeroUF64,
    offset: UF64,
}

impl LinearFn for NoiseFunction {
    fn evaluate(&self, x: f64) -> f64 {
        (self.scale * x + self.offset).sqrt()
    }
    fn reverse_evaluate(&self, y: f64) -> f64 {
        (y.powi(2) - self.offset) / self.scale
    }
    fn is_identity() -> bool {
        false
    }
}

pub struct Negative {
    pub stage: RawImageStage,

    /// Non locallized ASCII name
    model_name: String,
    /// Localized UTF-8 model name
    local_name: String,

    // The area of raw image that should be included in the final converted
    // image. This stems from extra pixels around the edges of the sensor
    // including both the black mask and some additional padding.

    // The default crop can be smaller than the "active" area which includes
    // the padding but not the black masked pixels.
    default_crop_size_h: URational,
    default_crop_size_v: URational,

    default_crop_origin_h: URational,
    default_crop_origin_v: URational,

    // Enhanced images can change the default crop, so we
    // need to keep around the original value.
    raw_default_crop_size_h: URational,
    raw_default_crop_size_v: URational,

    raw_default_crop_origin_h: URational,
    raw_default_crop_origin_v: URational,

    // Default user crop, in relative coordinates.
    default_user_crop_t: URational,
    default_user_crop_l: URational,
    default_user_crop_b: URational,
    default_user_crop_r: URational,

    // Default scale factors. Generally, 1.0 for square pixel cameras. They
    // can compensate for non-square pixels. The choice of exact values will
    // generally depend on what the camera does. These are particularly
    // interesting for the Nikon D1X and the Fuji diamond mosaic.
    default_scale_h: URational,
    default_scale_v: URational,

    // Enhanced images can change the default scale, so we
    // need to keep around the original value.
    raw_default_scale_h: URational,
    raw_default_scale_v: URational,

    // Best quality scale factor. Used for the Nikon D1X and Fuji cameras
    // to force everything to be a scale up rather than scale down. So,
    // generally this is 1.0 / min (fDefaultScaleH, fDefaultScaleV) but
    // this isn't used if the scale factors are only slightly different
    // from 1.0.
    best_quality_scale: URational,

    // Enhanced images can change the best quality scale, so we
    // need to keep around the original value.
    raw_best_quality_scale: URational,

    // Proxy image support.	 Remember certain sizes for the original image
    // this proxy was derived from.
    original_default_final_size: Point<i32>,
    original_best_quality_final_size: Point<i32>,

    original_default_crop_size_h: URational,
    original_default_crop_size_v: URational,

    // Scale factors used in demosaic algorithm (calculated).
    // Maps raw image coordinates to full image coordinates -- i.e.,
    // original image coordinates on raw sensor data to coordinates
    // in fStage3Image which is the output of the interpolation step.
    // So, if we downsample when interpolating, these numbers get
    // smaller.
    raw_to_full_scale_h: URational,
    raw_to_full_scale_v: URational,

    // Relative amount of noise at ISO 100. This is measured per camera model
    // based on looking at flat areas of color.
    baseline_noise: URational,

    // How much noise reduction has already been applied (0.0 to 1.0) to the
    // the raw image data?	0.0 = none, 1.0 = "ideal" amount--i.e. don't apply any
    // more by default.	 0/0 for unknown.
    noise_reduction_applied: URational,

    // Enhanced images can change the applied noise reduction, so we
    // need to keep around the original value.
    raw_noise_reduction_applied: URational,
    // Amount of noise for this negative (see dng_noise_profile for details).

    // dng_noise_profile fNoiseProfile;

    // // Enhanced images can change the noise profile, so we
    // // need to keep around the original value.

    // dng_noise_profile fRawNoiseProfile;

    // // Zero point for the exposure compensation slider. This reflects how
    // // the manufacturer sets up the camera and its conversions.

    // dng_srational fBaselineExposure;

    // // Relative amount of sharpening required. This is chosen per camera
    // // model based on how strong the anti-alias filter is on the camera
    // // and the quality of the lenses. This scales the sharpness slider
    // // value.

    // dng_urational fBaselineSharpness;

    // // Enhanced images can change the baseline sharpness, so we
    // // need to keep around the original value.

    // dng_urational fRawBaselineSharpness;

    // // Chroma blur radius (or 0/0 for auto). Set to 0/1 to disable
    // // chroma blurring.

    // dng_urational fChromaBlurRadius;

    // // Anti-alias filter strength (0.0 to 1.0).	 Used as a hint
    // // to the demosaic algorithms.

    // dng_urational fAntiAliasStrength;

    // // Linear response limit. The point at which the sensor goes
    // // non-linear and color information becomes unreliable. Used in
    // // the highlight-recovery logic.

    // dng_urational fLinearResponseLimit;

    // // Scale factor for shadows slider. The Fuji HDR cameras, for example,
    // // need a more sensitive shadow slider.

    // dng_urational fShadowScale;

    // // Colormetric reference.

    // uint32 fColorimetricReference;

    // // Is the stage 3 image floating point?

    // bool fFloatingPoint;

    // // Number of color channels for this image (e.g. 1, 3, or 4).

    // uint32 fColorChannels;

    // // Amount by which each channel has already been scaled. Some cameras
    // // have analog amplifiers on the color channels and these can result
    // // in different scalings per channel. This provides some level of
    // // analog white balancing. The Nikon D1 also did digital scaling but
    // // this caused problems with highlight recovery.

    // dng_vector fAnalogBalance;

    // // The "As Shot" neutral color coordinates in native camera space.
    // // This overrides fCameraWhiteXY if both are specified. This
    // // specifies the values per channel that would result in a neutral
    // // color for the "As Shot" case. This is generally supplied by
    // // the camera.

    // dng_vector fCameraNeutral;

    // // The "As Shot" white balance xy coordinates. Sometimes this is
    // // supplied by the camera. Sometimes the camera just supplies a name
    // // for the white balance.

    // dng_xy_coord fCameraWhiteXY;

    // // Individual camera calibrations.

    // // Camera data --> camera calibration --> "inverse" of color matrix
}
