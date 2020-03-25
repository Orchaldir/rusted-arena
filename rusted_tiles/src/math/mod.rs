pub mod color;
pub mod point;

use point::*;

pub fn get_index(x: u32, y: u32, size: Point) -> usize {
    ((y * size.x) + x) as usize
}

pub fn get_point(index: usize, size: Point) -> Point {
    xy(index as u32 % size.x, index as u32 / size.x)
}

pub fn get_corners(position: [f32; 2], size: [f32; 2]) -> [[f32; 2]; 4] {
    let corner00 = position;
    let corner10 = [position[0] + size[0], position[1]];
    let corner01 = [position[0], position[1] + size[1]];
    let corner11 = [position[0] + size[0], position[1] + size[1]];

    [corner00, corner10, corner01, corner11]
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: Point = Point { x: 2, y: 3 };

    #[test]
    fn test_get_index() {
        assert_eq!(get_index(0, 0, SIZE), 0);
        assert_eq!(get_index(1, 0, SIZE), 1);
        assert_eq!(get_index(0, 1, SIZE), 2);
        assert_eq!(get_index(1, 1, SIZE), 3);
        assert_eq!(get_index(0, 2, SIZE), 4);
        assert_eq!(get_index(1, 2, SIZE), 5);
    }

    #[test]
    fn test_get_point() {
        assert_eq!(get_point(0, SIZE), xy(0, 0));
        assert_eq!(get_point(1, SIZE), xy(1, 0));
        assert_eq!(get_point(2, SIZE), xy(0, 1));
        assert_eq!(get_point(3, SIZE), xy(1, 1));
        assert_eq!(get_point(4, SIZE), xy(0, 2));
        assert_eq!(get_point(5, SIZE), xy(1, 2));
    }
}
