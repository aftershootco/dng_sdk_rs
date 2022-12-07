use std::any::Any;

use crate::ifd::Ifd;
use crate::types::rational::{SRational, URational};

pub enum Types<'t> {
    Byte(u8),
    Ascii(&'t str),
    Short(u16),
    Long(u8),
    Rational(URational),
    SByte(i8),
    Undefined(&'t [u8]),
    SShort(i16),
    SLong(i32),
    SRational(SRational),
    Float(f32),
    Double(f64),
    IFD(Ifd),
    Unicode(&'t str),
    Complex(Box<dyn Any>),

    // Tag types added by big tiff
    Long8(u64),
    SLong8(i64),
    IFD8(Ifd),

    // Note that this is not an offical TIFF tag type, and should
    // not be used in TIFF/DNG files:
    HalfFloat(f16),
}

#[repr(transparent)]
pub struct f16(u16);
