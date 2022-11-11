#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Matrix<const ROWS: usize, const COLS: usize = ROWS> {
    pub data: [[f64; COLS]; ROWS],
}

impl<const ROWS: usize, const COLS: usize> Default for Matrix<ROWS, COLS> {
    fn default() -> Self {
        Self {
            data: [[0.0; COLS]; ROWS],
        }
    }
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    pub const fn new(data: [[f64; COLS]; ROWS]) -> Self {
        Self { data }
    }
}

/// Implementaions for square matrices
impl<const SIZE: usize> Matrix<SIZE, SIZE> {
    pub const fn identity() -> Self {
        let mut data = [[0.0; SIZE]; SIZE];
        let mut i = 0;
        loop {
            if i >= SIZE {
                break;
            }
            data[i][i] = 1.0;
            i += 1;
        }
        Self { data }
    }

    pub fn is_diagonal(&self) -> bool {
        for i in 0..SIZE {
            for j in 0..SIZE {
                if i != j && self.data[i][j] != 0.0 {
                    return false;
                }
            }
        }
        true
    }
}

impl<T, const ROWS: usize, const COLS: usize> std::ops::Mul<T> for Matrix<ROWS, COLS>
where
    f64: std::ops::Mul<T> + std::ops::MulAssign<T>,
    T: std::ops::Mul<f64, Output = f64> + Copy,
{
    type Output = Self;

    fn mul(mut self, rhs: T) -> Self::Output {
        for i in 0..ROWS {
            for j in 0..COLS {
                self.data[i][j] *= rhs;
            }
        }
        self
    }
}

impl<const ROWS: usize, const COLS: usize> std::ops::Add for Matrix<ROWS, COLS> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..ROWS {
            for j in 0..COLS {
                self.data[i][j] += rhs.data[i][j];
            }
        }
        self
    }
}

pub struct Vector<T, const DIM: usize> {
    pub data: [T; DIM],
}

impl<T, const LEN: usize> Vector<T, LEN> {
    #[inline]
    pub const fn new(data: [T; LEN]) -> Self {
        Self { data }
    }
}

impl<T, const LEN: usize> From<[T; LEN]> for Vector<T, LEN> {
    #[inline]
    fn from(data: [T; LEN]) -> Self {
        Self { data }
    }
}

// impl<T> From<[T; 3]> for Vector<T, 3> {
//     #[inline]
//     fn from(data: [T; 3]) -> Self {
//         Self { data }
//     }
// }

impl<T, const LEN: usize> std::ops::Index<usize> for Vector<T, LEN> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const LEN: usize> std::ops::IndexMut<usize> for Vector<T, LEN> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}
