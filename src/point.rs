use core::ops::Sub;
use std::ops::{Add, Mul};

use crate::traits::Hypotenuse;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point<T> {
    pub v: T,
    pub h: T,
}

impl<T> Point<T> {
    pub fn new(v: T, h: T) -> Self {
        Self { v, h }
    }
}

impl<T> Point<T> {
    pub fn length(&self) -> T
    where
        T: Hypotenuse,
    {
        self.v.hypotenuse(self.h)
    }
}

impl<T: Sub<Output = T> + Hypotenuse + Copy + Mul<Output = T> + Add<Output = T>> Point<T> {
    pub fn distance(&self, other: &Self) -> T {
        (*self - *other).length()
    }
    pub fn distance_squared(&self, other: &Self) -> T {
        let diff = *self - *other;
        (diff.v * diff.v) + (diff.h * diff.h)
    }
}

impl Point<f32> {
    pub fn scale(&self, scale: f32) -> Self {
        Self {
            v: self.v * scale,
            h: self.h * scale,
        }
    }
    pub fn normalize(&self) -> Self {
        let length = self.length();
        if length == 0.0 {
            return Self::new(0.0, 0.0);
        }
        self.scale(1.0 / length)
    }
}

impl Point<f64> {
    pub fn scale(&self, scale: f64) -> Self {
        Self {
            v: self.v * scale,
            h: self.h * scale,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Point<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            v: self.v - other.v,
            h: self.h - other.h,
        }
    }
}

impl TryFrom<Point<f64>> for Point<i32> {
    type Error = <i64 as TryInto<i32>>::Error;

    fn try_from(point: Point<f64>) -> Result<Self, Self::Error> {
        Ok(Self {
            v: (point.v.round() as i64).try_into()?,
            h: (point.h.round() as i64).try_into()?,
        })
    }
}
