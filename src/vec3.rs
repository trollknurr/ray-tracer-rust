use std::fmt;
use std::ops::{Mul, Add, Div, Sub};

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Vec3 {
    e0: f32,
    e1: f32,
    e2: f32
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} {} {}>", self.e0, self.e1, self.e2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f32) -> Vec3 {
        Vec3 {
            e0: self.e0 * f,
            e1: self.e1 * f,
            e2: self.e2 * f,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, v2: Vec3) -> Vec3 {
        Vec3 {
            e0: self.e0 + v2.e0,
            e1: self.e1 + v2.e1,
            e2: self.e2 + v2.e2,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, f: f32) -> Self::Output {
        self * (1. / f)
    }

}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z()
        )
    }
}

impl Vec3 {
    pub fn x(&self) -> f32 {self.e0}
    pub fn y(&self) -> f32 {self.e1}
    pub fn z(&self) -> f32 {self.e2}

    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3{
            e0: x,
            e1: y,
            e2: z
        }
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}