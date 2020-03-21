use crate::rendering::textured::*;

const N: u8 = 16;
const TC_C: f32 = 1.0 / N as f32;
const TC_SIZE: [f32; 2] = [TC_C, TC_C];
const INVALID_COLOR: [f32; 3] = [1.0, 0.08, 0.58];

#[derive(Default)]
pub struct AsciiBuilder {
    builder: TexturedTriangleBuilder,
}

impl AsciiBuilder {
    pub fn add_char(&mut self, position: [f32; 2], size: [f32; 2], c: char, color: [f32; 3]) {
        if c.is_ascii() {
            self.add_u8(position, size, c as u8, color);
        } else {
            self.add_u8(position, size, b'?', INVALID_COLOR);
        }
    }

    pub fn add_u8(&mut self, position: [f32; 2], size: [f32; 2], ascii: u8, color: [f32; 3]) {
        let row: u8 = ascii / N;
        let column: u8 = ascii % N;

        let tc = [column as f32 * TC_C, 1.0 - (row + 1) as f32 * TC_C];

        self.builder.add_tile(position, size, tc, TC_SIZE, color);
    }

    pub fn clear(&mut self) {
        self.builder.clear();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::testing::*;

    #[test]
    fn test_add_u8() {
        let mut builder = AsciiBuilder::default();

        builder.add_u8(P00, SIZE, b'A', COLOR);

        assert_textured_tile(builder.get(), POS, TC_A, COLOR);
    }

    #[test]
    fn test_add_char() {
        let mut builder = AsciiBuilder::default();

        builder.add_char(P00, SIZE, 'A', COLOR);

        assert_textured_tile(builder.get(), POS, TC_A, COLOR);
    }

    #[test]
    fn test_add_non_ascii_char() {
        let mut builder = AsciiBuilder::default();

        builder.add_char(P00, SIZE, '🎉', COLOR);

        assert_textured_tile(builder.get(), POS, TC_Q, INVALID_COLOR);
    }

    #[test]
    fn test_clear() {
        let mut builder = AsciiBuilder::default();

        builder.add_u8(P00, SIZE, b'W', COLOR);
        builder.clear();

        assert!(builder.get().is_empty());
    }
}
