use std::ops::{Add, AddAssign, Mul, Sub};

impl Vector {
    pub const fn new(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }

    pub fn delta(&self, other: &Vector) -> Vector {
        other - self
    }
    pub fn mirror(&self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<'a, 'b> Add<&'b Vector> for &'a Vector {
    type Output = Vector;

    fn add(self, rhs: &'b Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<isize> for Vector {
    type Output = Vector;

    fn mul(self, rhs: isize) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<(isize, isize)> for Vector {
    fn from(value: (isize, isize)) -> Self {
        Vector::new(value.0, value.1)
    }
}

#[derive(Default, PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
pub struct Vector {
    pub x: isize,
    pub y: isize,
}
