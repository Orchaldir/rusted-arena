use crate::rendering::textured::*;

const N: u8 = 16;
const TC_SIZE_UNIFORM: f32 = 1.0 / N as f32;
const TC_SIZE: [f32; 2] = [TC_SIZE_UNIFORM, TC_SIZE_UNIFORM];

#[derive(Default)]
pub struct AsciiBuilder {
    builder: TexturedTriangleBuilder,
}

impl AsciiBuilder {
    pub fn add_char(&mut self, position: [f32; 2], size: [f32; 2], c: char, color: [f32; 3]) {
        if c.is_ascii() {
            self.add_u8(position, size, c as u8, color);
        } else {
            self.add_u8(position, size, b'?', [1.0, 0.08, 0.58]);
        }
    }

    pub fn add_u8(&mut self, position: [f32; 2], size: [f32; 2], ascii: u8, color: [f32; 3]) {
        let row: u8 = ascii / N;
        let column: u8 = ascii % N;

        let tc = [
            column as f32 * TC_SIZE_UNIFORM,
            1.0 - (row + 1) as f32 * TC_SIZE_UNIFORM,
        ];

        self.builder.add_tile(position, size, tc, TC_SIZE, color);
    }

    pub fn get(&self) -> &Vec<TexturedVertex> {
        self.builder.get()
    }

    pub fn add_string(
        &mut self,
        position: [f32; 2],
        size: [f32; 2],
        string: &str,
        color: [f32; 3],
    ) {
        let mut new_p = position;
        for c in string.chars() {
            self.add_char(new_p, size, c, color);
            new_p[0] += size[0];
        }
    }
}
