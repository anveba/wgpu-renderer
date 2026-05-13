use std::{fmt::Display, mem::MaybeUninit};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Matrix<const M: usize, const N: usize, T> {
    values: [[T; M]; N],
}

impl<const M: usize, const N: usize, T> Matrix<M, N, T>
where
    T: Copy,
{
    pub fn from_array(values: &[[T; M]; N]) -> Self {
        Self { values: *values }
    }

    pub fn from_value(v: &T) -> Self {
        Self {
            values: [[*v; M]; N],
        }
    }

    pub fn transpose(&self) -> Matrix<N, M, T> {
        let mut res = unsafe { Matrix::<N, M, T>::new_uninit() };
        for i in 0..M {
            for j in 0..N {
                *res.index_mut(j, i) = *self.index(i, j);
            }
        }
        res
    }
}

impl<const M: usize, const N: usize, T> Matrix<M, N, T>
where
    T: Copy + num::traits::Zero,
{
    pub fn zeros() -> Self {
        Self {
            values: [[T::zero(); M]; N],
        }
    }
}

impl<const M: usize, const N: usize, T> Matrix<M, N, T>
where
    T: Copy + num::traits::One,
{
    pub fn ones() -> Self {
        Self {
            values: [[T::one(); M]; N],
        }
    }
}

impl<const N: usize, T> Matrix<N, N, T>
where
    T: Copy + num::traits::One + num::traits::Zero,
{
    pub fn identity() -> Self {
        let mut res = Self::zeros();
        for i in 0..N {
            *res.index_mut(i, i) = T::one();
        }
        res
    }
}

impl<const M: usize, const N: usize, T> Matrix<M, N, T> {
    pub unsafe fn new_uninit() -> Self {
        Self {
            values: MaybeUninit::uninit().assume_init(),
        }
    }

    pub fn index(&self, i: usize, j: usize) -> &T {
        &self.values[j][i]
    }

    pub fn index_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.values[j][i]
    }
}

impl<const M: usize, const N: usize, T> std::ops::Index<usize> for Matrix<M, N, T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
        assert!(i < M * N);
        let ptr = self.values.as_ptr() as *const T;
        unsafe { ptr.add(i).as_ref().unwrap_unchecked() }
    }
}

impl<const M: usize, const N: usize, T> std::ops::IndexMut<usize> for Matrix<M, N, T> {
    fn index_mut(&mut self, i: usize) -> &mut T {
        assert!(i < M * N);
        let ptr = self.values.as_mut_ptr() as *mut T;
        unsafe { ptr.add(i).as_mut().unwrap_unchecked() }
    }
}

impl<const M: usize, const N: usize, T> std::ops::Add<Matrix<M, N, T>> for Matrix<M, N, T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let mut res = unsafe { Self::new_uninit() };
        for i in 0..M {
            for j in 0..N {
                *res.index_mut(i, j) = *self.index(i, j) + *rhs.index(i, j);
            }
        }
        res
    }
}

impl<const M: usize, const N: usize, T> std::ops::AddAssign<Matrix<M, N, T>> for Matrix<M, N, T>
where
    T: std::ops::AddAssign + Copy,
{
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                *self.index_mut(i, j) += *rhs.index(i, j);
            }
        }
    }
}

impl<const M: usize, const N: usize, T> std::ops::Sub<Matrix<M, N, T>> for Matrix<M, N, T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let mut res = unsafe { Self::new_uninit() };
        for i in 0..M {
            for j in 0..N {
                *res.index_mut(i, j) = *self.index(i, j) - *rhs.index(i, j);
            }
        }
        res
    }
}

impl<const M: usize, const N: usize, T> std::ops::SubAssign<Matrix<M, N, T>> for Matrix<M, N, T>
where
    T: std::ops::SubAssign + Copy,
{
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..M {
            for j in 0..N {
                *self.index_mut(i, j) -= *rhs.index(i, j);
            }
        }
    }
}

impl<const M: usize, const N: usize, T> std::ops::Neg for Matrix<M, N, T>
where
    T: std::ops::Neg<Output = T> + Copy,
{
    type Output = Self;

    fn neg(self) -> Self {
        let mut res = unsafe { Self::new_uninit() };
        for i in 0..M {
            for j in 0..N {
                *res.index_mut(i, j) = -(*self.index(i, j));
            }
        }
        res
    }
}

impl<const M: usize, const N: usize, const P: usize, T> std::ops::Mul<Matrix<P, N, T>>
    for Matrix<M, P, T>
where
    T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + num::traits::Zero + Copy,
{
    type Output = Matrix<M, N, T>;

    fn mul(self, rhs: Matrix<P, N, T>) -> Matrix<M, N, T> {
        let mut res = Matrix::<M, N, T>::from_value(&T::zero());
        for i in 0..M {
            for k in 0..P {
                for j in 0..N {
                    *res.index_mut(i, j) = *res.index(i, j) + *self.index(i, k) * *rhs.index(k, j);
                }
            }
        }
        res
    }
}

impl<const M: usize, const N: usize, T> std::ops::Mul<T> for Matrix<M, N, T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Self;

    fn mul(self, v: T) -> Self {
        let mut res = unsafe { Self::new_uninit() };
        for i in 0..M {
            for j in 0..N {
                *res.index_mut(i, j) = *self.index(i, j) * v;
            }
        }
        res
    }
}

impl<const M: usize, const N: usize, T> Display for Matrix<M, N, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if M == 0 || N == 0 {
            f.write_str("[ ]")?;
            return Ok(());
        }
        f.write_str(&format!("["))?;
        for i in 0..M {
            for j in 0..N {
                f.write_str(&format!("{}", self.index(i, j)))?;
                if j < N - 1 && i < M - 1 {
                    f.write_str(&format!(", "))?;
                }
            }
            if i < M - 1 {
                f.write_str(&format!("\n "))?;
            }
        }
        f.write_str("]")?;
        Ok(())
    }
}
