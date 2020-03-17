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

    const SIZE: [f32; 2] = [0.5, 1.5];
    const P00: [f32; 2] = [2.0, 4.0];
    const P10: [f32; 2] = [2.5, 4.0];
    const P01: [f32; 2] = [2.0, 5.5];
    const P11: [f32; 2] = [2.5, 5.5];

    // char A has column 1 & row 11
    const A00: [f32; 2] = [0.0625, 0.6875];
    const A10: [f32; 2] = [0.125, 0.6875];
    const A01: [f32; 2] = [0.0625, 0.75];
    const A11: [f32; 2] = [0.125, 0.75];

    // char ? has column 15 & row 12
    const Q00: [f32; 2] = [0.9375, 0.75];
    const Q10: [f32; 2] = [1.0, 0.75];
    const Q01: [f32; 2] = [0.9375, 0.8125];
    const Q11: [f32; 2] = [1.0, 0.8125];

    const COLOR: [f32; 3] = [0.4, 0.5, 0.6];

    #[test]
    fn test_add_u8() {
        let mut builder = AsciiBuilder::default();

        builder.add_u8(P00, SIZE, b'A', COLOR);

        assert_u8(&mut builder, [A00, A10, A01, A11], COLOR);
    }

    #[test]
    fn test_add_char() {
        let mut builder = AsciiBuilder::default();

        builder.add_char(P00, SIZE, 'A', COLOR);

        assert_u8(&mut builder, [A00, A10, A01, A11], COLOR);
    }

    #[test]
    fn test_add_non_ascii_char() {
        let mut builder = AsciiBuilder::default();

        builder.add_char(P00, SIZE, '🎉', COLOR);

        assert_u8(&mut builder, [Q00, Q10, Q01, Q11], INVALID_COLOR);
    }

    fn assert_u8(builder: &mut AsciiBuilder, tcs: [[f32; 2]; 4], color: [f32; 3]) {
        let vertices = builder.get();
        assert_eq!(vertices.len(), 6);
        assert_tile(&vertices[0], P00, tcs[0], color);
        assert_tile(&vertices[1], P10, tcs[1], color);
        assert_tile(&vertices[2], P11, tcs[3], color);
        assert_tile(&vertices[3], P00, tcs[0], color);
        assert_tile(&vertices[4], P11, tcs[3], color);
        assert_tile(&vertices[5], P01, tcs[2], color);
    }

    fn assert_tile(vertex: &TexturedVertex, pos: [f32; 2], tc: [f32; 2], color: [f32; 3]) {
        assert_eq!(vertex.position, pos);
        assert_eq!(vertex.tc, tc);
        assert_eq!(vertex.color, color);
    }
}
