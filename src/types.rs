pub mod conversions;
pub mod non_zero_float;
pub mod rational;

pub const fn dng_char4(input: [u8; 4]) -> u32 {
    (input[0] as u32) << 24 | (input[1] as u32) << 16 | (input[2] as u32) << 8 | (input[3] as u32)
}

pub type Result<T> = std::result::Result<T, crate::errors::Error>;
