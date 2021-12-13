
use std::{f64,ops};

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Vector2 (pub f64, pub f64);

impl ops::Add for Vector2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
	Self(self.0+other.0, self.1+other.1)
    }
}

impl ops::Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
	Self(self.0-other.0, self.1-other.1)
    }
}

/* dot product for Vector2 */
impl ops::Mul for Vector2 {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
	self.0*other.0+self.1*other.1
    }
}

impl ops::Div<f64> for Vector2 {
    type Output = Self;
    fn div(self, other: f64) -> Self {
	Self(self.0/other,self.1/other)
    }
}

impl Vector2 {
    #[allow(dead_code)]
    pub fn square(self) -> f64 {
	self * self
    }

    /* length of Vector2 */
    #[allow(dead_code)]
    pub fn length(self) -> f64 {
	self.square().sqrt()
    }
}

#[derive(Debug,Copy,Clone,PartialEq)]
pub struct Vector3 (pub f64, pub f64, pub f64);

impl ops::Add for Vector3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
	Self(self.0+other.0, self.1+other.1, self.2+other.2)
    }
}

impl ops::Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
	Self(self.0-other.0, self.1-other.1, self.2-other.2)
    }
}

/* dot product for Vector3 */
impl ops::Mul for Vector3 {
    type Output = f64;
    fn mul(self, other: Self) -> f64 {
	self.0*other.0+self.1*other.1+self.2*other.2
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;
    fn mul(self, other: Vector3) -> Vector3 {
	Vector3(self*other.0, self*other.1, self*other.2)
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, other:f64) -> Vector3 {
	Vector3(self.0/other, self.0/other, self.0/other)
    }
}

impl Vector3 {
    #[allow(dead_code)]
    pub fn square(self) -> f64 {
	self * self
    }

    /* length of Vector3 */
    #[allow(dead_code)]
    pub fn length(self) -> f64 {
	self.square().sqrt()
    }

    /* Cross Product of Vector3 */
    pub fn cross_product(self, other: Self) -> Self {
	let x = self.1 * other.2 - self.2 * other.1;
	let y = self.2 * other.0 - self.0 * other.2;
	let z = self.0 * other.1 - self.1 * other.0;
	Self(x, y, z)
    }
}

pub fn cross_product(a:Vector3, b:Vector3) -> Vector3 {
    Vector3(a.1*b.2-a.2*b.1, a.2*b.0-a.0*b.2, a.0*b.1-a.1*b.0)
}
