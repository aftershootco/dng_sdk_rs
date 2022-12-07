use std::cmp::Ordering;
use std::ops::{Add, Div, Mul, Sub};

/// Unsigned float type
/// the value should be greater than 0.0
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UF64(f64);

impl Add<f64> for UF64 {
    type Output = f64;

    fn add(self, rhs: f64) -> Self::Output {
        self.0 + rhs
    }
}

impl Sub<f64> for UF64 {
    type Output = f64;

    fn sub(self, rhs: f64) -> Self::Output {
        self.0 - rhs
    }
}

impl Add<UF64> for f64 {
    type Output = f64;

    fn add(self, rhs: UF64) -> Self::Output {
        self + rhs.0
    }
}

impl Sub<UF64> for f64 {
    type Output = f64;

    fn sub(self, rhs: UF64) -> Self::Output {
        self - rhs.0
    }
}

impl PartialOrd<f64> for UF64 {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl PartialEq<f64> for UF64 {
    fn eq(&self, other: &f64) -> bool {
        self.0.eq(other)
    }
}

impl UF64 {
    pub fn new(value: f64) -> Option<Self> {
        if value < 0.0 {
            None
        } else {
            Some(Self(value))
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonZeroUF64(UF64);

impl Mul<f64> for NonZeroUF64 {
    type Output = f64;

    fn mul(self, rhs: f64) -> Self::Output {
        self.0 .0 * rhs
    }
}

impl Div<f64> for NonZeroUF64 {
    type Output = f64;

    fn div(self, rhs: f64) -> Self::Output {
        self.0 .0 / rhs
    }
}

impl Div<NonZeroUF64> for f64 {
    type Output = f64;

    fn div(self, rhs: NonZeroUF64) -> Self::Output {
        self / rhs.0 .0
    }
}

impl NonZeroUF64 {
    pub fn new(value: UF64) -> Option<Self> {
        if value == 0.0 {
            None
        } else {
            Some(Self(value))
        }
    }

    /// Creates a non-zero without checking whether the value is non-zero. This results in undefined behaviour if the value is zero.
    /// # Safety
    /// The value must not be zero.
    pub unsafe fn new_unchecked(value: UF64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> f64 {
        self.0 .0
    }
}
