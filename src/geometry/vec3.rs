use std::ops::{Add, AddAssign, Sub, Mul, MulAssign, Div, DivAssign, Neg, Index, IndexMut};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {

    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn zero() -> Self {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
        
    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
}

// Overriding Vec3 negation
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 { e: [-self.e[0], -self.e[1], -self.e[2]] }
    }
}

// Overriding Vec3 immutable indexing
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index < 3 {
            &self.e[index]
        }
        else {
            panic!("Index {} out of bounds for Vec3", index);
        }
    }
}

// Overriding Vec3 mutable indexing
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        if index < 3 {
            &mut self.e[index]
        }
        else {
            panic!("Index {} out of bounds for Vec3", index);
        }
    }
}

// Overriding Vec3 add assign
impl AddAssign for Vec3 {
    fn add_assign(&mut self, v: Vec3) {
        self.e[0] += v.e[0];
        self.e[1] += v.e[1];
        self.e[2] += v.e[2];
    }
}

// Overriding Vec3 mult assign
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, t: f64) {
        self.e[0] *= t;
        self.e[1] *= t;
        self.e[2] *= t;
    }
}

// Overriding Vec3 div assign
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, t: f64) {
        if t == 0.0 {
            panic!("Cannot divide by zero!");
        }
        else {
            self.e[0] /= t;
            self.e[1] /= t;
            self.e[2] /= t;
        }
    }
}

// Overriding Vec3 add
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] + v.e[0], 
            self.e[1] + v.e[1], 
            self.e[2] + v.e[2]
        )
    }
}

// Overriding Vec3 sub
impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] - v.e[0], 
            self.e[1] - v.e[1], 
            self.e[2] - v.e[2]
        )
    }
}

// Overriding Vec3 mul
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3::new(
            self.e[0] * v.e[0], 
            self.e[1] * v.e[1], 
            self.e[2] * v.e[2]
        )
    }
}

// Overriding Vec3 mul for scalar doubles, (Vec3 * f64)
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f64) -> Vec3 {
        Vec3::new(
            self.e[0] * t, 
            self.e[1] * t, 
            self.e[2] * t
        )
    }
}

// (f64 * Vec3)
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        v * self
    }
}

// Overriding Vec3 div for scalar doubles
impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        self * (1.0 / t)
    }
}

// Output formatter
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.e[0] * v.e[0] +
    u.e[1] * v.e[1] +
    u.e[2] * v.e[2]
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3::new(u.e[1] * v.e[2] - u.e[2] * v.e[1],
        u.e[2] * v.e[0] - u.e[0] * v.e[2],
        u.e[0] * v.e[1] - u.e[1] * v.e[0]
    )   
}

pub fn unit_v(v: Vec3) -> Vec3 {
    v / v.length()
}

pub type Point3 = Vec3;
