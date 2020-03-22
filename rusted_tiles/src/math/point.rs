#[derive(PartialEq, Copy, Clone)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn get_area(self) -> usize {
        (self.x * self.y) as usize
    }
}
