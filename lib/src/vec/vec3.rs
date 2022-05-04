use super::Vec;
use std::ops;

pub type Vec3 = Vec<3>;
pub type Point = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { e: [x, y, z] }
    }

    pub fn x(&self) -> i32 {
        return self.e[0];
    }

    pub fn y(&self) -> i32 {
        return self.e[1];
    }

    pub fn z(&self) -> i32 {
        return self.e[2];
    }

    pub fn length_squared(&self) -> i32 {
        let x = self.x();
        let y = self.y();
        let z = self.z();

        let x_2 = x * x;
        let y_2 = y * y;
        let z_2 = z * z;

        x_2 + y_2 + z_2
    }

    pub fn length(&self) -> i32 {
        (self.length_squared() as f64).sqrt() as i32 + 1
    }

    pub fn dot(v: Vec3, u: Vec3) -> i32 {
        let x = u.x() * v.x();
        let y = u.y() * v.y();
        let z = u.z() * v.z();
        x + y + z
    }

    pub fn cross(u: Vec3, v: Vec3) -> Self {
        let x = u.y() * v.z() - u.z() * v.y();
        let y = u.z() * v.x() - u.x() * v.z();
        let z = u.x() * v.y() - u.y() * v.x();
        Self::new(x, y, z)
    }

    pub fn unit_vector(v: Vec3) -> Self {
        let length = v.length();
        v / length
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        let x = -self.x();
        let y = -self.y();
        let z = -self.z();
        Self::new(x, y, z)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();
        Self::new(x, y, z)
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        let z = self.z() - rhs.z();
        Self::new(x, y, z)
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        let z = self.z() * rhs.z();
        Self::new(x, y, z)
    }
}

impl ops::Mul<i32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self {
        let x = self.x() * rhs;
        let y = self.y() * rhs;
        let z = self.z() * rhs;
        Self::new(x, y, z)
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self {
        let x = self.x() / rhs.x();
        let y = self.y() / rhs.y();
        let z = self.z() / rhs.z();
        Self::new(x, y, z)
    }
}

impl ops::Div<i32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: i32) -> Self {
        let x = self.x() / rhs;
        let y = self.y() / rhs;
        let z = self.z() / rhs;
        Self::new(x, y, z)
    }
}
