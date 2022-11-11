use std::num::{NonZeroI32, NonZeroU32};

#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Rational<T, NZ> {
    pub numerator: T,
    pub denominator: NZ,
}

pub type IRational = Rational<i32, NonZeroI32>;
pub type URational = Rational<u32, NonZeroU32>;
