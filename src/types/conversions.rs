use crate::coord::Coord;
use crate::matrix::Vector;

impl Coord {
    /// This will be made const once
    /// https://github.com/rust-lang/rust/issues/67792
    /// stablizes
    fn from_xyz(v: Vector<f64, 3>) -> Self {
        let x = v[0];
        let y = v[1];
        let z = v[2];
        let sum = x + y + z;
        if sum > 0.0 {
            Self {
                x: x / sum,
                y: y / sum,
            }
        } else {
            Coord::D50_XY_COORD
        }
    }
}

impl From<Vector<f64, 3>> for Coord {
    fn from(v: Vector<f64, 3>) -> Self {
        Coord::from_xyz(v)
    }
}

impl Vector<f64, 3> {
    const fn from_xy(c: Coord) -> Self {
        todo!()
    }
}

impl From<Coord> for Vector<f64, 3> {
    fn from(c: Coord) -> Self {
        Self::from_xy(c)
    }
}
