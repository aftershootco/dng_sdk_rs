use crate::coord::Coord;
use crate::errors::{Error, ErrorKind};
use crate::Result;

#[derive(Debug, Clone)]
pub struct Temperature {
    pub temperature: f64,
    pub tint: f64,
}

pub struct RUVT {
    pub r: f64,
    pub u: f64,
    pub v: f64,
    pub t: f64,
}

impl RUVT {
    pub const fn new(r: f64, u: f64, v: f64, t: f64) -> RUVT {
        RUVT { r, u, v, t }
    }
}
#[allow(warnings)]
const K_TINT_SCALE: f64 = -3000.0;

const K_TEMP_TABLE: [RUVT; 31] = [
    RUVT::new(0.0, 0.18006, 0.26352, -0.24341),
    RUVT::new(10.0, 0.18066, 0.26589, -0.25479),
    RUVT::new(20.0, 0.18133, 0.26846, -0.26876),
    RUVT::new(30.0, 0.18208, 0.27119, -0.28539),
    RUVT::new(40.0, 0.18293, 0.27407, -0.30470),
    RUVT::new(50.0, 0.18388, 0.27709, -0.32675),
    RUVT::new(60.0, 0.18494, 0.28021, -0.35156),
    RUVT::new(70.0, 0.18611, 0.28342, -0.37915),
    RUVT::new(80.0, 0.18740, 0.28668, -0.40955),
    RUVT::new(90.0, 0.18880, 0.28997, -0.44278),
    RUVT::new(100.0, 0.19032, 0.29326, -0.47888),
    RUVT::new(125.0, 0.19462, 0.30141, -0.58204),
    RUVT::new(150.0, 0.19962, 0.30921, -0.70471),
    RUVT::new(175.0, 0.20525, 0.31647, -0.84901),
    RUVT::new(200.0, 0.21142, 0.32312, -1.0182),
    RUVT::new(225.0, 0.21807, 0.32909, -1.2168),
    RUVT::new(250.0, 0.22511, 0.33439, -1.4512),
    RUVT::new(275.0, 0.23247, 0.33904, -1.7298),
    RUVT::new(300.0, 0.24010, 0.34308, -2.0637),
    RUVT::new(325.0, 0.24702, 0.34655, -2.4681),
    RUVT::new(350.0, 0.25591, 0.34951, -2.9641),
    RUVT::new(375.0, 0.26400, 0.35200, -3.5814),
    RUVT::new(400.0, 0.27218, 0.35407, -4.3633),
    RUVT::new(425.0, 0.28039, 0.35577, -5.3762),
    RUVT::new(450.0, 0.28863, 0.35714, -6.7262),
    RUVT::new(475.0, 0.29685, 0.35823, -8.5955),
    RUVT::new(500.0, 0.30505, 0.35907, -11.324),
    RUVT::new(525.0, 0.31320, 0.35968, -15.628),
    RUVT::new(550.0, 0.32129, 0.36011, -23.325),
    RUVT::new(575.0, 0.32931, 0.36038, -40.770),
    RUVT::new(600.0, 0.33724, 0.36051, -116.45),
];

impl Temperature {
    pub fn from_coord(xy: &Coord) -> Result<Self> {
        // convert to uv space
        let u: f64 = 2.0 * xy.x / (1.5 - xy.x + 6.0 * xy.y);
        let v: f64 = 3.0 * xy.y / (1.5 - xy.x + 6.0 * xy.y);

        // Search for line pair coordinate is between.
        let mut last_dt: f64 = 0.0;
        let mut last_dv: f64 = 0.0;
        let mut last_du: f64 = 0.0;

        for (index, ktable) in K_TEMP_TABLE.iter().skip(1).enumerate() {
            let du: f64 = 1.0;
            let dv: f64 = ktable.t;
            let len: f64 = (dv.powi(2) + 1.0).sqrt();
            let mut du = du / len;
            let mut dv = dv / len;

            // Find delta from black body point to test coordinate.
            let uu = u - ktable.u;
            let vv = v - ktable.v;
            // Find distance above or below line.
            let mut dt = -uu * dv + vv * du;

            // If below line, we have found line pair.
            if dt <= 0.0 || index == 30 {
                // Find the fractional weight of the two lines
                if dt > 0.0 {
                    dt = 0.0;
                }
                dt = dt.abs();
                let f = if index == 1 { 0.0 } else { dt / (last_dt + dt) };

                // Interpolate the temperature
                // fTemperature = 1.0E6 / (kTempTable[index - 1].r * f +
                //                         kTempTable[index].r * (1.0 - f));
                let temp = 1.0e6 / (K_TEMP_TABLE[index - 1].r * f + ktable.r * (1.0 - f));

                // Find delta from black body point to test coordinate.

                // uu = u - (kTempTable[index - 1].u * f + kTempTable[index].u * (1.0 - f));

                // vv = v - (kTempTable[index - 1].v * f + kTempTable[index].v * (1.0 - f));
                let uu = u - (K_TEMP_TABLE[index - 1].u * f + ktable.u * (1.0 - f));
                let vv = v - (K_TEMP_TABLE[index - 1].v * f + ktable.v * (1.0 - f));
                // Interpolate vectors along slope.

                // du = du * (1.0 - f) + last_du * f;
                // dv = dv * (1.0 - f) + last_dv * f;
                du = du * (1.0 - f) + last_du * f;
                dv = dv * (1.0 - f) + last_dv * f;
                let len = (dv.powi(2) + du.powi(2)).sqrt();
                du /= len;
                dv /= len;

                // Find the distance along slope
                let tint = (uu * du + vv * dv) * K_TINT_SCALE;
                return Ok(Self {
                    temperature: temp,
                    tint,
                });
            }
            // Try next line pair.

            last_dt = dt;

            last_du = du;
            last_dv = dv;
        }
        Err(Error::new(ErrorKind::InvalidTemperature))
    }
}
