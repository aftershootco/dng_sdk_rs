use crate::matrix::Vector;

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Coord {
    pub x: f64,
    pub y: f64,
}

impl Default for Coord {
    fn default() -> Self {
        Coord::new(0.0, 0.0)
    }
}

impl Coord {
    pub const fn new(x: f64, y: f64) -> Coord {
        Coord { x, y }
    }
    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }

    pub fn is_valid(&self) -> bool {
        self.x > 0.0 && self.y > 0.0
    }

    pub fn is_invalid(&self) -> bool {
        !self.is_valid()
    }
}

impl From<(f64, f64)> for Coord {
    fn from((x, y): (f64, f64)) -> Coord {
        Coord::new(x, y)
    }
}

impl std::ops::Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, other: Coord) -> Coord {
        Coord {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f64> for Coord {
    type Output = Coord;

    fn mul(self, other: f64) -> Coord {
        Coord {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::ops::Div<f64> for Coord {
    type Output = Coord;

    fn div(self, other: f64) -> Coord {
        Coord {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl std::ops::Mul<Coord> for Coord {
    type Output = f64;

    fn mul(self, other: Coord) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

impl Coord {
    pub fn xyz_to_xy(input: Vector<f64, 3>) -> Coord {
        let total = input.data[0] + input.data[1] + input.data[2];
        if total > 0.0 {
            Coord {
                x: input.data[0] / total,
                y: input.data[1] / total,
            }
        } else {
            Coord::D50_XY_COORD
        }
    }
    pub fn xy_to_xyz(mut input: Self) -> Vector<f64, 3> {
        input.x = input.x.clamp(Self::MIN, Self::MAX);
        input.y = input.y.clamp(Self::MIN, Self::MAX);

        if input.x + input.y > Self::MAX {
            let scale = Self::MAX / (input.x + input.y);
            input.x *= scale;
            input.y *= scale;
        }

        Vector::new([input.x / input.y, 1.0, (1.0 - input.x - input.y) / input.y])
    }
}

/// Standard xy coordinates
impl Coord {
    const MAX: f64 = 0.999999;
    const MIN: f64 = 0.000001;
    pub const STD_A_XY_COORD: Coord = Coord::new(0.4476, 0.4074);
    pub const D50_XY_COORD: Coord = Coord::new(0.3457, 0.3585);
    pub const D55_XY_COORD: Coord = Coord::new(0.3324, 0.3474);
    pub const D65_XY_COORD: Coord = Coord::new(0.3127, 0.3290);
    pub const D75_XY_COORD: Coord = Coord::new(0.2990, 0.3149);
    pub const PCS_TO_XY: Coord = Coord::D50_XY_COORD;
    pub const PCS_TO_XYZ: fn() -> Vector<f64, 3> = || Coord::xy_to_xyz(Coord::PCS_TO_XY);
}
