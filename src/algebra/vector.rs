use crate::algebra::Matrix;

pub type ColVec<const D: usize, T> = Matrix<D, 1, T>;
pub type RowVec<const D: usize, T> = Matrix<1, D, T>;

pub type ColVecf<const D: usize> = ColVec<D, f32>;
pub type RowVecf<const D: usize> = RowVec<D, f32>;

pub type Vec2 = ColVecf<2>;
pub type Vec3 = ColVecf<3>;
pub type Vec4 = ColVecf<4>;

impl<const D: usize, T> ColVec<D, T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
    T: num::traits::Zero,
{
    pub fn dot(v1: Self, v2: Self) -> T {
        let mut res = T::zero();
        for i in 0..D {
            res = res + v1[i] * v2[i];
        }
        res
    }

    pub fn norm_squared(self) -> T {
        Self::dot(self, self)
    }
}

impl<const D: usize, T> ColVec<D, T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: num::traits::Float,
{
    pub fn norm(self) -> T {
        self.norm_squared().sqrt()
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        let values = [x, y];
        Self::from_array(&[values])
    }

    pub fn x(&self) -> f32 {
        self[0]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self[0]
    }

    pub fn y(&self) -> f32 {
        self[1]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self[1]
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        let values = [x, y, z];
        Self::from_array(&[values])
    }

    pub fn x(&self) -> f32 {
        self[0]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self[0]
    }

    pub fn y(&self) -> f32 {
        self[1]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self[1]
    }

    pub fn z(&self) -> f32 {
        self[2]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self[2]
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Self {
        let x = v1.y() * v2.z() - v2.y() * v1.z();
        let y = v2.x() * v1.z() - v1.x() * v2.z();
        let z = v1.x() * v2.y() - v2.x() * v1.y();
        Vec3::new(x, y, z)
    }

    pub fn unit_orthogonal(&self) -> Vec3 {
        let a = 1.0f32.copysign(self.z());
        let b = 1.0 / (a + self.z());
        Vec3::new(
            a - self.x() * self.x() * b,
            -self.x() * self.y() * b,
            -self.x(),
        )
    }
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        let values = [x, y, z, w];
        Self::from_array(&[values])
    }

    pub fn x(&self) -> f32 {
        self[0]
    }

    pub fn x_mut(&mut self) -> &mut f32 {
        &mut self[0]
    }

    pub fn y(&self) -> f32 {
        self[1]
    }

    pub fn y_mut(&mut self) -> &mut f32 {
        &mut self[1]
    }

    pub fn z(&self) -> f32 {
        self[2]
    }

    pub fn z_mut(&mut self) -> &mut f32 {
        &mut self[2]
    }

    pub fn w(&self) -> f32 {
        self[3]
    }

    pub fn w_mut(&mut self) -> &mut f32 {
        &mut self[3]
    }
}
