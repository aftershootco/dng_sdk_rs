use std::ops::Mul;

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
    pub const fn row_size() -> usize {
        ROWS
    }
    pub const fn col_size() -> usize {
        COLS
    }
    pub const fn new(data: [[f64; COLS]; ROWS]) -> Self {
        Self { data }
    }
    pub const fn row(&self, row: usize) -> [f64; COLS] {
        self.data[row]
    }
    pub const fn col(&self, col: usize) -> [f64; ROWS] {
        let mut data = [0.0; ROWS];
        let mut row = 0;
        loop {
            if row == ROWS {
                break data;
            }
            data[row] = self.data[0][col];
            row += 1;
        }
    }
}

/// Implementaions for square matrices
impl<const SIZE: usize> Matrix<SIZE> {
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

    pub fn determinant(&self) -> f64 {
        match SIZE {
            1 => self.data[0][0],
            2 => self.data[0][0] * self.data[1][1] - self.data[0][1] * self.data[1][0],
            3 => {
                self.data[0][0] * self.data[1][1] * self.data[2][2]
                    + self.data[0][1] * self.data[1][2] * self.data[2][0]
                    + self.data[0][2] * self.data[1][0] * self.data[2][1]
                    - self.data[0][2] * self.data[1][1] * self.data[2][0]
                    - self.data[0][1] * self.data[1][0] * self.data[2][2]
                    - self.data[0][0] * self.data[1][2] * self.data[2][1]
            }

            _ => {
                unimplemented!("Unsupported matrix size");
            }
        }
    }

    pub const fn invert(&self) -> Self {
        let mut data = [[0.0; SIZE]; SIZE];
        let mut i = 0;
        loop {
            if i >= SIZE {
                break;
            }
            let mut j = 0;
            loop {
                if j >= SIZE {
                    break;
                }
                data[i][j] = self.data[j][i];
                j += 1;
            }
            i += 1;
        }
        Self { data }
    }

    pub const fn cofactor<const C_SIZE: usize>(&self, row: usize, col: usize) -> Matrix<C_SIZE> {
        let mut data = [[0.0; C_SIZE]; C_SIZE];
        let mut i = 0;
        loop {
            if i >= C_SIZE {
                break;
            }
            let mut j = 0;
            loop {
                if j >= C_SIZE {
                    break;
                }
                data[i][j] = self.data[i + (i >= row) as usize][j + (j >= col) as usize];
                j += 1;
            }
            i += 1;
        }
        Matrix { data }
    }
}

/// Implement Matrix multiplication where we can only multiply where the matrices are AxB and BxC
impl<const A: usize, const B: usize, const C: usize> Mul<Matrix<B, C>> for Matrix<A, B> {
    type Output = Matrix<A, C>;

    fn mul(self, rhs: Matrix<B, C>) -> Self::Output {
        let mut data = [[0.0; C]; A];
        (0..A).for_each(|i| {
            for j in 0..C {
                for k in 0..B {
                    data[i][j] += self.data[i][k] * rhs.data[k][j];
                }
            }
        });

        Self::Output { data }
    }
}

// impl Matrix<3> {
//     fn mul(self, rhs: Matrix<3>) -> Matrix<3> {
//         let mut data = [[0.0; 3]; 3];
//         (0..3).for_each(|i| {
//             for j in 0..3 {
//                 for k in 0..3 {
//                     data[i][j] += self.data[i][k] * rhs.data[k][j];
//                 }
//             }
//         });

//         Self { data }
//     }
// }

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Vector<T, const DIM: usize> {
    pub(crate) data: [T; DIM],
}

impl<T: Copy, const LEN: usize> Copy for Vector<T, LEN> {}

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

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T, const LEN: usize> std::ops::IndexMut<usize> for Vector<T, LEN> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<Matrix<ROWS, COLS>> for Vector<f64, ROWS> {
    type Output = Vector<f64, COLS>;

    #[inline]
    fn mul(self, rhs: Matrix<ROWS, COLS>) -> Self::Output {
        let mut data = [0.0; COLS];
        for elem in data.iter_mut() {
            for (i, row) in rhs.data.iter().enumerate() {
                *elem += self.data[i] * row[0];
            }
        }
        Self::Output { data }
    }
}

impl<const ROWS: usize, const COLS: usize> Mul<Vector<f64, COLS>> for Matrix<ROWS, COLS> {
    type Output = Vector<f64, ROWS>;

    #[inline]
    fn mul(self, rhs: Vector<f64, COLS>) -> Self::Output {
        let mut data = [0.0; ROWS];
        for elem in data.iter_mut() {
            for (i, row) in self.data.iter().enumerate() {
                *elem += row[i] * rhs.data[i];
            }
        }
        Self::Output { data }
    }
}
