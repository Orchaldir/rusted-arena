use crate::rendering::textured::*;

const N: u8 = 16;
const TC_SIZE_UNIFORM: f32 = 1.0 / N as f32;
const TC_SIZE: [f32; 2] = [TC_SIZE_UNIFORM, TC_SIZE_UNIFORM];

#[derive(Default)]
pub struct AsciiBuilder {
    builder: TexturedTriangleBuilder,
}

impl AsciiBuilder {
    pub fn add_u8(&mut self, position: [f32; 2], size: [f32; 2], symbol: u8, color: [f32; 3]) {
        let row: u8 = symbol / N;
        let column: u8 = symbol % N;

        let tc = [
            column as f32 * TC_SIZE_UNIFORM,
            1.0 - (row + 1) as f32 * TC_SIZE_UNIFORM,
        ];

        self.builder.add_tile(position, size, tc, TC_SIZE, color);
    }

    pub fn get(&self) -> &Vec<TexturedVertex> {
        self.builder.get()
    }
}
