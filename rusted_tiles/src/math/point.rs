use std::ops::{Add, Mul};

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

pub fn xy(x: u32, y: u32) -> Point {
    Point { x, y }
}

pub const ZERO: Point = Point { x: 0, y: 0 };

impl Point {
    pub fn get_area(self) -> usize {
        (self.x * self.y) as usize
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}
