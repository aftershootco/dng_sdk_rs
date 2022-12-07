use crate::illuminant::Illuminant;
use crate::matrix::Matrix;
use crate::types::rational::SRational;

pub struct CameraProfileInfo {
    pub big_endian: bool,

    pub color_planes: u32,

    pub calibration_illuminant1: u32,
    pub calibration_illuminant2: u32,
    pub calibration_illuminant3: u32,

    pub illuminant_data1: Illuminant,
    pub illuminant_data2: Illuminant,
    pub illuminant_data3: Illuminant,

    pub color_matrix1: Matrix<3>,
    pub color_matrix2: Matrix<3>,
    pub color_matrix3: Matrix<3>,

    pub forward_matrix1: Matrix<3>,
    pub forward_matrix2: Matrix<3>,
    pub forward_matrix3: Matrix<3>,

    pub reduction_matrix1: Matrix<3>,
    pub reduction_matrix2: Matrix<3>,
    pub reduction_matrix3: Matrix<3>,

    pub profile_calibration_signature: String,

    pub profile_name: String,

    pub profile_copyright: String,

    pub embed_policy: u32,

    pub profile_hues: u32,
    pub profile_sats: u32,
    pub profile_vals: u32,

    pub hue_sat_deltas1_offset: u64,
    pub hue_sat_deltas1_count: u32,

    pub hue_sat_deltas2_offset: u64,
    pub hue_sat_deltas2_count: u32,

    pub hue_sat_deltas3_offset: u64,
    pub hue_sat_deltas3_count: u32,

    pub hue_sat_map_encoding: u32,

    pub look_table_hues: u32,
    pub look_table_sats: u32,
    pub look_table_vals: u32,

    pub look_table_offset: u64,
    pub look_table_count: u32,

    pub look_table_encoding: u32,

    pub baseline_exposure_offset: SRational,

    pub default_black_render: u32,

    pub tone_curve_offset: u64,
    pub tone_curve_count: u32,

    pub unique_camera_model: String,
}
