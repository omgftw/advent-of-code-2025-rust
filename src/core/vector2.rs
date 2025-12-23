use std::ops::Sub;
use num_integer::Roots;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Vector2 {
    pub x: i64,
    pub y: i64,
}

impl Sub for Vector2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Vector2 {
    pub fn distance(self, other: Self) -> i64 {
        let diff = self - other;
        let x = diff.x.pow(2);
        let y = diff.y.pow(2);
        (x + y).sqrt().abs()
    }
}
