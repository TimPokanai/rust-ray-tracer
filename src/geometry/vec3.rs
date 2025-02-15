use std::ops::{Add, AddAssign, Sub, Mul, Div, Neg, Index, IndexMut};
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
