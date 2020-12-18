use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign, Neg};
use std::cmp::PartialEq;

#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
        }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn dot(self, other: Vec2) -> f32 {
        (self.x * other.x) + (self.y * other.y)
    }

    pub fn length(self) -> f32 {
        self.dot(self).sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    pub fn length_recip(self) -> f32 {
        self.length().recip()
    }

    pub fn normalize(self) -> Vec2 {
        self * self.length_recip()
    }

    pub fn abs(self) -> Vec2 {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns a vector perpendicular to this one, in the direction of `hint`
    pub fn perpendicular(self, hint: Vec2) -> Vec2 {
        let p = Self::new(self.y, -self.x);
        if p.normalize().dot(hint.normalize()) > 0.0 {
            p
        } else {
            Self::new(-self.y, self.x)
        }
    }

    pub fn perpendicular_cw(self) -> Vec2 {
        Self::new(self.y, -self.x)
    }

    pub fn perpendicular_ccw(self) -> Vec2 {
        Self::new(-self.y, self.x)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vec2 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl MulAssign for Vec2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div for Vec2 {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl DivAssign for Vec2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl Add<f32> for Vec2 {
    type Output = Self;

    fn add(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

impl AddAssign<f32> for Vec2 {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl Sub<f32> for Vec2 {
    type Output = Self;

    fn sub(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}

impl SubAssign<f32> for Vec2 {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}


impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        Vec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign<f32> for Vec2 {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        (*self - *other).abs().length() < std::f32::EPSILON
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perpendicular() {
        let a = Vec2::new(0.0, 1.0);
        let hint = Vec2::new(1.0, 0.0);
        let p = a.perpendicular(hint);

        assert_eq!(p, Vec2::new(1.0, 0.0));

        let a = Vec2::new(0.0, 1.0);
        let hint = Vec2::new(-1.0, 0.0);
        let p = a.perpendicular(hint);

        assert_eq!(p, Vec2::new(-1.0, 0.0));
    }
}
