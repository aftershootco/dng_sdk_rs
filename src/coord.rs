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

impl std::ops::Add for Coord {
    type Output = Coord;

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Sub for Coord {
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

impl std::ops::Mul<Coord> for Coord {
    type Output = f64;

    fn mul(self, other: Coord) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

/// Standard xy coordinates
impl Coord {
    pub const STD_A_XY_COORD: Coord = Coord::new(0.4476, 0.4074);
    pub const D50_XY_COORD: Coord = Coord::new(0.3457, 0.3585);
    pub const D55_XY_COORD: Coord = Coord::new(0.3324, 0.3474);
    pub const D65_XY_COORD: Coord = Coord::new(0.3127, 0.3290);
    pub const D75_XY_COORD: Coord = Coord::new(0.2990, 0.3149);
}
