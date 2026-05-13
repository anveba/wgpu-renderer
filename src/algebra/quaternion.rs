use super::Matrix;

pub struct Quaternion<T>
where
    T: num::traits::Float,
{
    s: T,
    v: Matrix<3, 1, T>,
}

impl<T> Quaternion<T>
where
    T: num::traits::Float,
{
    pub fn identity() -> Self {
        Self {
            s: T::one(),
            v: Matrix::<3, 1, T>::zeros(),
        }
    }
}
