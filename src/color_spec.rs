// dng_xy_coord dng_color_spec::NeutralToXY (const dng_vector &neutral)
// 	{

// 	const uint32 kMaxPasses = 30;

// 	if (fChannels == 1)
// 		{

// 		return PCStoXY ();

// 		}

// 	dng_xy_coord last = D50_xy_coord ();

// 	for (uint32 pass = 0; pass < kMaxPasses; pass++)
// 		{

// 		dng_matrix xyzToCamera = FindXYZtoCamera (last);

// 		dng_xy_coord next = XYZtoXY (Invert (xyzToCamera) * neutral);

// 		if (Abs_real64 (next.x - last.x) +
// 			Abs_real64 (next.y - last.y) < 0.0000001)
// 			{

// 			return next;

// 			}

// 		// If we reach the limit without converging, we are most likely
// 		// in a two value oscillation.	So take the average of the last
// 		// two estimates and give up.

// 		if (pass == kMaxPasses - 1)
// 			{

// 			next.x = (last.x + next.x) * 0.5;
// 			next.y = (last.y + next.y) * 0.5;

// 			}

// 		last = next;

// 		}

// 	return last;

// 	}

use crate::coord::Coord;
use crate::errors::{Error, ErrorKind};
use crate::illuminant::Illuminant;
use crate::matrix::{Matrix, Vector};
use crate::temparature::Temperature;

pub struct ColorSpec<
    const ILML: usize,
    const ROWS: usize,
    const COLS: usize = ROWS,
    const LEN: usize = ROWS,
> {
    pub channels: u32,

    pub temparature1: f64,
    pub temparature2: f64,

    pub light: [Illuminant; 3],

    pub color_matrix: [Matrix<ROWS, COLS>; ILML],

    pub forward_matrix: [Option<Matrix<ROWS, COLS>>; ILML],

    pub reduction_matrix: [Option<Matrix<ROWS, COLS>>; ILML],

    pub camera_calibration: [Matrix<ROWS, COLS>; ILML],

    pub analog_balance: Matrix<ROWS, COLS>,

    pub white_xy: Coord,

    pub camera_white: Vector<f64, LEN>,
    pub camera_to_pcs: Matrix<ROWS, COLS>,

    pub pcs_to_camera: Matrix<ROWS, COLS>,
}

impl<const ILML: usize, const ROWS: usize, const COLS: usize, const LEN: usize>
    ColorSpec<ILML, ROWS, COLS, LEN>
{
    pub fn neutral_to_xy(&self, neutral: Vector<f64, LEN>) -> Coord {
        const MAX_PASS: u32 = 30;

        if self.channels == 1 {
            // return self.pcs_to_xy(); // Huh ?
        }

        let last = Coord::D50_XY_COORD;

        for pass in 0..MAX_PASS {}
        todo!()
    }

    pub fn find_xyz_to_camera(
        &self,
        white: &Coord,
        mut forward_matrix: Option<Matrix<ROWS, COLS>>,
        mut reduction_matrix: Option<Matrix<ROWS, COLS>>,
        mut camera_calibration: Option<Matrix<ROWS, COLS>>,
    ) -> crate::Result<Matrix<ROWS, COLS>> {
        match ILML {
            1 | 2 => self.find_xyz_to_camera_single_or_dual(
                white,
                &mut forward_matrix,
                &mut reduction_matrix,
                &mut camera_calibration,
            ),
            3 => self.find_xyz_to_camera_triple(
                white,
                &mut forward_matrix,
                &mut reduction_matrix,
                &mut camera_calibration,
            ),
            c => Err(Error::new(ErrorKind::InvalidColorSpec(c))),
        }
    }

    fn find_xyz_to_camera_single_or_dual(
        &self,
        white: &Coord,
        forward_matrix: &mut Option<Matrix<ROWS, COLS>>,
        reduction_matrix: &mut Option<Matrix<ROWS, COLS>>,
        camera_calibration: &mut Option<Matrix<ROWS, COLS>>,
    ) -> crate::Result<Matrix<ROWS, COLS>> {
        let td = Temperature::from_coord(white)?;
        let g: f64 = match td.temperature {
            t if t <= self.temparature1 => 1.0,
            t if t >= self.temparature2 => 0.0,
            t => {
                let inv_t: f64 = 1.0 / t;
                (inv_t - (1.0 / self.temparature2))
                    / ((1.0 / self.temparature1) - (1.0 / self.temparature2))
            }
        };

        // Interpolate the color_matrix

        let color_matrix = match g {
            g if g >= 1.0 => self.color_matrix[0],
            g if g <= 0.0 => self.color_matrix[1],
            g => self.color_matrix[0] * g + self.color_matrix[1] * (1.0 - g),
        };

        // Interpolate the forward_matrix if any
        if let Some(ref mut forward_matrix) = forward_matrix {
            // if let (Some(fm1), Some(fm2)) = (self.forward_matrix[0], self.forward_matrix[1]) {
            //     match g {
            //         g if g >= 1.0 => *forward_matrix = fm1,
            //         g if g <= 0.0 => *forward_matrix = fm2,
            //         g => *forward_matrix = fm1 * g + fm2 * (1.0 - g),
            //     }
            // } else if let Some(fm1) = self.forward_matrix[0] {
            //     *forward_matrix = fm1
            // } else if let Some(fm2) = self.forward_matrix[1] {
            //     *forward_matrix = fm2
            // }
            match (self.forward_matrix[0], self.forward_matrix[1]) {
                (Some(fm1), Some(fm2)) => match g {
                    g if g >= 1.0 => *forward_matrix = fm1,
                    g if g <= 0.0 => *forward_matrix = fm2,
                    g => *forward_matrix = fm1 * g + fm2 * (1.0 - g),
                },
                (Some(fm1), None) => *forward_matrix = fm1,
                (None, Some(fm2)) => *forward_matrix = fm2,
                (None, None) => {}
            }
        }

        // Interpolate the reduction_matrix if any
        if let Some(ref mut reduction_matrix) = reduction_matrix {
            // if let (Some(rm1), Some(rm2)) = (self.reduction_matrix[0], self.reduction_matrix[1]) {
            //     match g {
            //         g if g >= 1.0 => *reduction_matrix = rm1,
            //         g if g <= 0.0 => *reduction_matrix = rm2,
            //         g => *reduction_matrix = rm1 * g + rm2 * (1.0 - g),
            //     }
            // } else if let Some(rm1) = self.reduction_matrix[0] {
            //     *reduction_matrix = rm1
            // } else if let Some(rm2) = self.reduction_matrix[1] {
            //     *reduction_matrix = rm2
            // }
            match (self.reduction_matrix[0], self.reduction_matrix[1]) {
                (Some(rm1), Some(rm2)) => match g {
                    g if g >= 1.0 => *reduction_matrix = rm1,
                    g if g <= 0.0 => *reduction_matrix = rm2,
                    g => *reduction_matrix = rm1 * g + rm2 * (1.0 - g),
                },
                (Some(rm1), None) => *reduction_matrix = rm1,
                (None, Some(rm2)) => *reduction_matrix = rm2,
                (None, None) => {}
            }
        }

        // Interpolate the camera_calibration
        if let Some(ref mut camera_calibration) = camera_calibration {
            match g {
                g if g >= 1.0 => *camera_calibration = self.camera_calibration[0],
                g if g <= 0.0 => *camera_calibration = self.camera_calibration[1],
                g => {
                    *camera_calibration =
                        self.camera_calibration[0] * g + self.camera_calibration[1] * (1.0 - g)
                }
            }
        }

        // return the interpolated color_matrix
        Ok(color_matrix)
    }

    fn find_xyz_to_camera_triple(
        &self,
        white: &Coord,
        forward_matrix: &mut Option<Matrix<ROWS, COLS>>,
        reduction_matrix: &mut Option<Matrix<ROWS, COLS>>,
        camera_calibration: &mut Option<Matrix<ROWS, COLS>>,
    ) -> crate::Result<Matrix<ROWS, COLS>> {
        let weights = Illuminant::calculate_triple_illuminant_weights(white, &self.light)?;
        todo!()
    }
}
