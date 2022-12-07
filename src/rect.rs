use std::ops::{Add, Shl, Shr};

use num::traits::{CheckedAdd, CheckedSub};

pub trait Contains<T> {
    fn contains(&self, other: &T) -> bool;
}

pub trait Width<T> {
    fn width(&self) -> T;
}

pub trait Height<T> {
    fn height(&self) -> T;
}

use crate::point::Point;
pub type IRect = Rect<i32>;
#[derive(Default, Eq, PartialEq, PartialOrd, Ord)]
pub struct Rect<T> {
    pub top: T,
    pub left: T,
    pub bottom: T,
    pub right: T,
}

impl Contains<IRect> for IRect {
    #[inline]
    fn contains(&self, other: &IRect) -> bool {
        self.top <= other.top
            && self.left <= other.left
            && self.bottom >= other.bottom
            && self.right >= other.right
    }
}

impl Contains<Point<i32>> for IRect {
    #[inline]
    fn contains(&self, other: &Point<i32>) -> bool {
        self.top <= other.v && self.left <= other.h && self.bottom > other.v && self.right > other.h
    }
}

impl TryFrom<(u32, u32)> for IRect {
    type Error = <i32 as TryFrom<u32>>::Error;
    fn try_from((width, height): (u32, u32)) -> Result<Self, Self::Error> {
        Ok(Self {
            top: 0,
            left: 0,
            bottom: height.try_into()?,
            right: width.try_into()?,
        })
    }
}

impl TryFrom<Point<i32>> for IRect {
    type Error = std::convert::Infallible;
    fn try_from(point: Point<i32>) -> Result<Self, Self::Error> {
        Ok(Self {
            top: 0,
            left: 0,
            bottom: point.v,
            right: point.h,
        })
    }
}

impl<T: num::Float + num::Zero> Width<T> for Rect<T> {
    fn width(&self) -> T {
        T::zero().max(self.right - self.left)
    }
}

impl<T: num::CheckedSub> Width<Option<T>> for Rect<T> {
    fn width(&self) -> Option<T> {
        self.right.checked_sub(&self.left)
    }
}

impl<T: num::Float + num::Zero> Height<T> for Rect<T> {
    fn height(&self) -> T {
        T::zero().max(self.bottom - self.top)
    }
}

impl<T: num::CheckedSub> Height<Option<T>> for Rect<T> {
    fn height(&self) -> Option<T> {
        self.bottom.checked_sub(&self.top)
    }
}

impl<T: std::ops::Add<Output = T> + Copy> std::ops::Add<Point<T>> for Rect<T> {
    type Output = Self;
    fn add(self, rhs: Point<T>) -> Self::Output {
        Self {
            top: self.top + rhs.v,
            left: self.left + rhs.h,
            bottom: self.bottom + rhs.v,
            right: self.right + rhs.h,
        }
    }
}

impl<T: std::ops::Sub<Output = T> + Copy> std::ops::Sub<Point<T>> for Rect<T> {
    type Output = Self;
    fn sub(self, rhs: Point<T>) -> Self::Output {
        Self {
            top: self.top - rhs.v,
            left: self.left - rhs.h,
            bottom: self.bottom - rhs.v,
            right: self.right - rhs.h,
        }
    }
}

impl<T> Rect<T> {
    pub fn new(top: T, left: T, bottom: T, right: T) -> Option<Self>
    where
        T: CheckedSub,
    {
        bottom.checked_sub(&top)?;
        right.checked_sub(&left)?;
        Some(Self {
            top,
            left,
            bottom,
            right,
        })
    }
    pub fn width(&self) -> Option<T>
    where
        T: CheckedSub,
    {
        self.right.checked_sub(&self.left)
    }

    pub fn height(&self) -> Option<T>
    where
        T: CheckedSub,
    {
        self.bottom.checked_sub(&self.top)
    }
    pub fn top_left(&self) -> Point<T>
    where
        T: Copy,
    {
        Point::new(self.top, self.left)
    }
    pub fn top_right(&self) -> Point<T>
    where
        T: Copy,
    {
        Point::new(self.top, self.right)
    }
    pub fn bottom_left(&self) -> Point<T>
    where
        T: Copy,
    {
        Point::new(self.bottom, self.left)
    }
    pub fn bottom_right(&self) -> Point<T>
    where
        T: Copy,
    {
        Point::new(self.bottom, self.right)
    }
    pub fn size(&self) -> Option<Point<T>>
    where
        T: CheckedSub,
    {
        let width = self.width()?;
        let height = self.height()?;
        Some(Point::new(width, height))
    }
    pub fn long_side(&self) -> Option<T>
    where
        T: CheckedSub + Ord,
    {
        let width = self.width()?;
        let height = self.height()?;
        Some(width.max(height))
    }
    pub fn short_side(&self) -> Option<T>
    where
        T: CheckedSub + Ord,
    {
        let width = self.width()?;
        let height = self.height()?;
        Some(width.min(height))
    }
    pub fn diagonal(&self) -> Option<f64>
    where
        T: CheckedSub + Ord + Into<f64>,
    {
        let width: f64 = self.width()?.into();
        let height: f64 = self.height()?.into();
        Some(width.hypot(height))
    }
    pub fn transpose(&self) -> Self
    where
        T: Copy,
    {
        Self {
            top: self.left,
            left: self.top,
            bottom: self.right,
            right: self.bottom,
        }
    }
    pub fn padded(&self, padding: T) -> Option<Self>
    where
        T: CheckedSub + CheckedAdd,
    {
        let top = self.top.checked_add(&padding)?;
        let left = self.left.checked_add(&padding)?;
        let bottom = self.bottom.checked_sub(&padding)?;
        let right = self.right.checked_sub(&padding)?;
        Some(Self {
            top,
            left,
            bottom,
            right,
        })
    }
}

impl<T> Rect<T>
where
    T: Add<Output = T> + Shr<i32, Output = T> + Shl<i32, Output = T> + num::CheckedSub + Copy,
{
    pub fn half_rect(&self) -> Option<Self> {
        Some(Self {
            top: self.top,
            left: self.left,
            bottom: self.top + (self.width()? >> 1),
            right: self.left + (self.height()? >> 1),
        })
    }
    pub fn double_rect(&self) -> Option<Self> {
        Some(Self {
            top: self.top,
            left: self.left,
            bottom: self.top + (self.width()? << 1),
            right: self.left + (self.height()? << 1),
        })
    }
}
