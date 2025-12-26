use std::cmp::Ordering;
use std::ops::Sub;
// use num_integer::Roots;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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

impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Vector2 {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lexicographic ordering: compare x first, then y
        match self.x.cmp(&other.x) {
            Ordering::Equal => self.y.cmp(&other.y),
            other => other,
        }
    }
}

impl Vector2 {
    // pub fn distance(self, other: Self) -> i64 {
    //     let diff = self - other;
    //     let x = diff.x.pow(2);
    //     let y = diff.y.pow(2);
    //     (x + y).sqrt().abs()
    // }

    pub fn area(self, other: Self) -> i64 {
        // Add 1 to offset 0-based coordinates
        let x_delta = (self.x - other.x).abs() + 1;
        let y_delta = (self.y - other.y).abs() + 1;
        x_delta * y_delta
    }
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}
