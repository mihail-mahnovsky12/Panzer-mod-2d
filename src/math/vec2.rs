use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct mVec2 {
    pub x: f64,
    pub y: f64,
}

impl mVec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Add for mVec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for mVec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul for mVec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
