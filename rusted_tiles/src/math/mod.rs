pub mod color;

pub fn get_index(position: [u32; 2], size: [u32; 2]) -> usize {
    ((position[1] * size[0]) + position[0]) as usize
}

pub fn get_corners(position: [f32; 2], size: [f32; 2]) -> [[f32; 2]; 4] {
    let corner00 = position;
    let corner10 = [position[0] + size[0], position[1]];
    let corner01 = [position[0], position[1] + size[1]];
    let corner11 = [position[0] + size[0], position[1] + size[1]];

    [corner00, corner10, corner01, corner11]
}
