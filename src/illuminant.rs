use crate::coord::Coord;
use crate::matrix::Vector;
use crate::point::Point;
use crate::tag::values::LightSource;
use crate::temparature::Temperature;
use crate::traits::{LinearFn, Normalize, SmoothStep};
use crate::types::rational::URational;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum IlluminantType {
    WhiteXY,
    Spectrum,
}

pub struct Illuminant {
    pub illuminant_type: IlluminantType,
    pub derived_white: Coord,

    pub white_x: URational,
    pub white_y: URational,

    pub min_lambda: URational,
    pub lambda_spacing: URational,

    pub spectrum: Vec<URational>,
}

impl Illuminant {
    pub fn set_white_xy(&mut self, white_xy: Coord) {
        const MIN: f64 = 0.000001;
        const MAX: f64 = 0.999999;
        let x = URational::from(white_xy.x);
        let y = URational::from(white_xy.y);

        // if  < MIN || x > MAX || y < MIN || y > MAX {
        //     panic!("Invalid white point");
        // }
    }
    pub fn illuminant_data(light: &LightSource, other: Self) -> Self {
        match light {
            LightSource::StandardLightA | LightSource::Tungsten => {
                // Illuminant::illuminant_data_a(other)
            }
            _ => todo!(),
        }
        todo!()
    }
    pub fn calculate_triple_illuminant_weights(
        white: &Coord,
        light: &[Self; 3],
    ) -> crate::Result<Vector<f64, 3>> {
        // Compute distance from white to each of the lights in a scaled
        // (1/temperature, tint) space, then map the distances to weights.

        let whites = [
            light[0].derived_white,
            light[1].derived_white,
            light[2].derived_white,
        ];

        let tt: [Temperature; 4] = [
            Temperature::from_coord(white)?,
            Temperature::from_coord(&whites[0])?,
            Temperature::from_coord(&whites[1])?,
            Temperature::from_coord(&whites[2])?,
        ];

        // Map the intrinsics

        const TINT_SCALE: f64 = 1.0 / 200.0;

        let pt: [Point<f64>; 4] = [
            Point::new(
                tt[0].tint * TINT_SCALE,
                (1500.0 / tt[0].temperature).min(1.0),
            ),
            Point::new(
                tt[1].tint * TINT_SCALE,
                (1500.0 / tt[1].temperature).min(1.0),
            ),
            Point::new(
                tt[2].tint * TINT_SCALE,
                (1500.0 / tt[2].temperature).min(1.0),
            ),
            Point::new(
                tt[3].tint * TINT_SCALE,
                (1500.0 / tt[3].temperature).min(1.0),
            ),
        ];

        // Compute squared differences and convert to weights

        const MIN_DIST: f64 = 1.0e-8;
        let weights: [f64; 3] = [
            1.0 / ((pt[0]).distance_squared(&pt[1]) + MIN_DIST),
            1.0 / ((pt[0]).distance_squared(&pt[2]) + MIN_DIST),
            1.0 / ((pt[0]).distance_squared(&pt[3]) + MIN_DIST),
        ];

        let weights = weights.normalize();

        // Smooth

        let weights = weights.smooth_step();

        // Supress small weights

        // Map min_weight to zero
        // Map 1 to 1
        // Clip result to [0, 1]
        const MIN_WEIGHT: f64 = 0.02;
        const SCALE: f64 = 1.0 / (1.0 - MIN_WEIGHT);

        let weights = [
            ((weights[0] - MIN_WEIGHT) * SCALE).clamp(0.0, 1.0),
            ((weights[1] - MIN_WEIGHT) * SCALE).clamp(0.0, 1.0),
            ((weights[2] - MIN_WEIGHT) * SCALE).clamp(0.0, 1.0),
        ];

        let mut weights = weights.normalize();

        weights[2] = (1.0 - weights[0] - weights[1]).max(0.0);
        Ok(weights.into())
    }
}

pub struct MapTemperature {
    pub temperature: f64,
    pub tint: f64,
}

impl LinearFn for MapTemperature {
    fn is_identity() -> bool {
        false
    }

    fn evaluate(&self, x: f64) -> f64 {
        (1500.0 / x).min(1.0)
    }

    fn reverse_evaluate(&self, _: f64) -> f64 {
        unimplemented!("Impossible to reverse evaluate MapTemperature")
    }
}
