use crate::limits::{
    MAX_BLACK_PATTERN, MAX_CFA_PATTERN, MAX_COLOR_PLANES, MAX_MASKED_AREAS, MAX_SAMPLES_PER_PIXEL,
};
use crate::rect::Rect;
use crate::types::rational::URational;

pub struct Ifd {
    uses_new_sub_file_type: bool,

    new_sub_file_type: u32,

    image_width: u32,
    image_length: u32,

    bits_per_sample: [u32; MAX_SAMPLES_PER_PIXEL],

    compression: u32,

    predictor: u32,

    photometric_interpretation: u32,

    fill_order: u32,

    orientation: u32,
    orientation_type: u32,
    orientation_offset: u64,
    orientation_big_endian: bool,

    samples_per_pixel: u32,

    planar_configuration: u32,

    x_resolution: f64,
    y_resolution: f64,

    resolution_unit: u32,

    uses_strips: bool,
    uses_tiles: bool,

    tile_width: u32,
    tile_length: u32,

    // enum
    //     {
    //     kMaxTileInfo = 32
    //     };
    tile_offsets_type: u32,
    tile_offsets_count: u32,
    tile_offsets_offset: u64,
    tile_offset: [u64; Self::MAX_TILE_INFO],
    // uint32 fTileByteCountsType;
    tile_byte_counts_type: u32,
    // uint32 fTileByteCountsCount;
    tile_byte_counts_count: u32,
    // uint64 fTileByteCountsOffset;
    tile_byte_counts_offset: u64,
    // uint64 fTileByteCount [kMaxTileInfo];
    tile_byte_count: [u64; Self::MAX_TILE_INFO],
    // uint32 fSubIFDsType;
    sub_ifds_type: u32,
    // uint32 fSubIFDsCount;
    sub_ifds_count: u32,
    // uint64 fSubIFDsOffset;
    sub_ifds_offset: u64,

    // uint32 fExtraSamplesCount;
    extra_samples_count: u32,
    // uint32 fExtraSamples [kMaxSamplesPerPixel];
    extra_samples: [u32; MAX_SAMPLES_PER_PIXEL],
    // uint32 fSampleFormat [kMaxSamplesPerPixel];
    sample_format: [u32; MAX_SAMPLES_PER_PIXEL],

    // uint32 fJPEGTablesCount;
    jpeg_tables_count: u32,
    // uint64 fJPEGTablesOffset;
    jpeg_tables_offset: u64,

    // uint64 fJPEGInterchangeFormat;
    jpeg_interchange_format: u64,
    // uint32 fJPEGInterchangeFormatLength;
    jpeg_interchange_format_length: u32,

    // real64 fYCbCrCoefficientR;
    ycbcr_coefficient_r: f64,
    // real64 fYCbCrCoefficientG;
    ycbcr_coefficient_g: f64,
    // real64 fYCbCrCoefficientB;
    ycbcr_coefficient_b: f64,
    // uint32 fYCbCrSubSampleH;
    ycbcr_sub_sample_h: u32,
    // uint32 fYCbCrSubSampleV;
    ycbcr_sub_sample_v: u32,

    // uint32 fYCbCrPositioning;
    ycbcr_positioning: u32,

    // real64 fReferenceBlackWhite [6];
    reference_black_white: [f64; 6],

    // uint32 fCFARepeatPatternRows;
    cfa_repeat_pattern_rows: u32,
    // uint32 fCFARepeatPatternCols;
    cfa_repeat_pattern_cols: u32,

    // uint8 fCFAPattern [kMaxCFAPattern] [kMaxCFAPattern];
    cfa_pattern: [[u8; MAX_CFA_PATTERN]; MAX_CFA_PATTERN],
    // uint8 fCFAPlaneColor [kMaxColorPlanes];
    cfa_plane_color: [u8; MAX_COLOR_PLANES],
    // uint32 fCFALayout;
    cfa_layout: u32,

    // uint32 fLinearizationTableType;
    linearization_table_type: u32,
    // uint32 fLinearizationTableCount;
    linearization_table_count: u32,
    // uint64 fLinearizationTableOffset;
    linearization_table_offset: u64,

    // uint32 fBlackLevelRepeatRows;
    black_level_repeat_rows: u32,
    // uint32 fBlackLevelRepeatCols;
    black_level_repeat_cols: u32,
    // real64 fBlackLevel [kMaxBlackPattern] [kMaxBlackPattern] [kMaxColorPlanes];
    black_level: [[[f64; MAX_COLOR_PLANES]; MAX_BLACK_PATTERN]; MAX_BLACK_PATTERN],
    // uint32 fBlackLevelDeltaHType;
    black_level_delta_h_type: u32,
    // uint32 fBlackLevelDeltaHCount;
    black_level_delta_h_count: u32,
    // uint64 fBlackLevelDeltaHOffset;
    black_level_delta_h_offset: u64,

    // uint32 fBlackLevelDeltaVType;
    black_level_delta_v_type: u32,
    // uint32 fBlackLevelDeltaVCount;
    black_level_delta_v_count: u32,
    // uint64 fBlackLevelDeltaVOffset;
    black_level_delta_v_offset: u64,
    // real64 fWhiteLevel [kMaxColorPlanes];
    white_level: [f64; MAX_COLOR_PLANES],

    // dng_urational fDefaultScaleH;
    default_scale_h: URational,
    // dng_urational fDefaultScaleV;
    default_scale_v: URational,

    // dng_urational fBestQualityScale;
    best_quality_scale: URational,

    // dng_urational fDefaultCropOriginH;
    default_crop_origin_h: URational,
    // dng_urational fDefaultCropOriginV;
    default_crop_origin_v: URational,

    // dng_urational fDefaultCropSizeH;
    default_crop_size_h: URational,
    // dng_urational fDefaultCropSizeV;
    default_crop_size_v: URational,

    // dng_urational fDefaultUserCropT;
    default_user_crop_t: URational,
    // dng_urational fDefaultUserCropL;
    default_user_crop_l: URational,
    // dng_urational fDefaultUserCropB;
    default_user_crop_b: URational,
    // dng_urational fDefaultUserCropR;
    default_user_crop_r: URational,

    // uint32 fBayerGreenSplit;
    bayer_green_split: u32,

    // dng_urational fChromaBlurRadius;
    chroma_blur_radius: URational,

    // dng_urational fAntiAliasStrength;
    anti_alias_strength: URational,

    // dng_rect fActiveArea;
    active_area: Rect<i32>,

    // uint32	 fMaskedAreaCount;
    masked_area_count: u32,
    // dng_rect fMaskedArea [kMaxMaskedAreas];
    masked_area: [Rect<i32>; MAX_MASKED_AREAS],
    // uint32 fRowInterleaveFactor;
    row_interleave_factor: u32,

    // uint32 fSubTileBlockRows;
    sub_tile_block_rows: u32,
    // uint32 fSubTileBlockCols;
    sub_tile_block_cols: u32,

    // dng_preview_info fPreviewInfo;
    preview_info: u32, //("dng_preview_info")

    // uint32 fOpcodeList1Count;
    opcode_list1_count: u32,
    // uint64 fOpcodeList1Offset;
    opcode_list1_offset: u64,

    // uint32 fOpcodeList2Count;
    opcode_list2_count: u32,
    // uint64 fOpcodeList2Offset;
    opcode_list2_offset: u64,
    // uint32 fOpcodeList3Count;
    // uint64 fOpcodeList3Offset;

    // dng_noise_profile fNoiseProfile;

    // dng_string fEnhanceParams;

    // dng_urational fBaselineSharpness;

    // dng_urational fNoiseReductionApplied;

    // bool fLosslessJPEGBug16;

    // uint32 fSampleBitShift;

    // uint64 fThisIFD;
    // uint64 fNextIFD;

    // int32 fCompressionQuality;

    // bool fPatchFirstJPEGByte;

    // dng_string fSemanticName;
    // dng_string fSemanticInstanceID;
    // std::shared_ptr<const dng_memory_block> fSemanticXMP;
    // uint32 fMaskSubArea [4];

    // std::shared_ptr<const dng_gain_table_map> fProfileGainTableMap;

    // std::shared_ptr<const dng_masked_rgb_tables> fMaskedRGBTables;
}

impl Ifd {
    const MAX_TILE_INFO: usize = 32;
}
