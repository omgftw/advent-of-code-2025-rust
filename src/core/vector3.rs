use std::ops::Sub;
use num_integer::Roots;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vector3 {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Sub for Vector3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Vector3 {
    pub fn distance(self, other: Self) -> i64 {
        let diff = self - other;
        let x = diff.x.pow(2);
        let y = diff.y.pow(2);
        let z = diff.z.pow(2);
        (x + y + z).sqrt().abs()
    }
}
