#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum Orientation {
    Normal = 0,
    Rotate90CW = 1,
    Rotate180 = 2,
    Rotate90CCW = 3,
    Mirror = 4,
    Mirror90CW = 5,
    Mirror180 = 6,
    Mirror90CCW = 7,
    Unknown = 8,
}

impl Orientation {
    pub fn from_tiff(tiff: u32) -> Self {
        match tiff {
            1 => Orientation::Normal,
            2 => Orientation::Mirror,
            3 => Orientation::Rotate180,
            4 => Orientation::Mirror180,
            5 => Orientation::Mirror90CCW,
            6 => Orientation::Rotate90CW,
            7 => Orientation::Mirror90CW,
            8 => Orientation::Rotate90CCW,
            9 => Orientation::Unknown,
            _ => Orientation::Normal,
        }
    }
    pub fn to_tiff(&self) -> u32 {
        match self {
            Orientation::Normal => 1,
            Orientation::Mirror => 2,
            Orientation::Rotate180 => 3,
            Orientation::Mirror180 => 4,
            Orientation::Mirror90CCW => 5,
            Orientation::Rotate90CW => 6,
            Orientation::Mirror90CW => 7,
            Orientation::Rotate90CCW => 8,
            Orientation::Unknown => 9,
        }
    }
    pub fn flip_d(self) -> bool {
        self as u32 & 1 != 0
    }

    pub fn flip_h(self) -> bool {
        if self as u32 & 4 != 0 {
            self as u32 & 2 == 0
        } else {
            self as u32 & 2 != 0
        }
    }

    pub fn flip_v(self) -> bool {
        if self as u32 & 4 != 0 {
            self.flip_d() == self.flip_h()
        } else {
            self.flip_d() != self.flip_h()
        }
    }
    pub fn is_mirrored(self) -> bool {
        self as u32 & 4 != 0
    }
}



