use std::num::{NonZeroI32, NonZeroU32};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rational<T, NZ> {
    pub numerator: T,
    pub denominator: NZ,
}

pub type SRational = Rational<i32, NonZeroI32>;
pub type URational = Rational<u32, NonZeroU32>;

impl<T, NZ> Rational<T, NZ> {
    pub fn new(numerator: impl Into<T>, denominator: impl Into<NZ>) -> Self {
        Self {
            numerator: numerator.into(),
            denominator: denominator.into(),
        }
    }
}

impl URational {
    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator.get() as f64
    }
}

impl SRational {
    pub fn as_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator.get() as f64
    }
}

/// If the numerator and denominator are both specified
/// and denominator is not zero, then the rational number
/// if calculated
impl From<(f64, u32)> for URational {
    fn from((x, mut dd): (f64, u32)) -> Self {
        if x <= 0.0 {
            return Self::new(0_u32, unsafe { NonZeroU32::new_unchecked(1) });
        }
        if dd == 0 {
            match x {
                x if x >= u16::MAX as f64 => dd = 1, // Since the number is so large we can ignore the denominator
                x if x >= 1.0 => dd = u16::MAX as u32, // Since the number is withing the range use a
                // large enough value that we don't hit the
                // limits
                _ => dd = (u16::MAX as u32).pow(2), // Since the number is so small we can ignore the denominator
            }
        }
        dbg!(x, dd);
        Self {
            numerator: (x * dd as f64).round() as u32,
            denominator: unsafe { NonZeroU32::new_unchecked(dd) },
        }
    }
}

impl From<f64> for URational {
    fn from(x: f64) -> Self {
        Self::from((x, 0))
    }
}

#[test]
fn test_delta() {
    assert_eq!(URational::from(0.000_011).as_f64(), 0.000_001);
}
