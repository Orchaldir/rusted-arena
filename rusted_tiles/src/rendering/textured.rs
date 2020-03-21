use crate::math::color::*;
use crate::math::get_corners;

#[derive(Copy, Clone)]
pub struct TexturedVertex {
    pub position: [f32; 2],
    pub tc: [f32; 2], // texture coordinates
    pub color: [f32; 3],
}

#[derive(Default)]
pub struct TexturedTriangleBuilder {
    vertices: Vec<TexturedVertex>,
}

impl TexturedTriangleBuilder {
    pub fn add_triangle(
        &mut self,
        a: [f32; 2],
        b: [f32; 2],
        c: [f32; 2],
        tc_a: [f32; 2],
        tc_b: [f32; 2],
        tc_c: [f32; 2],
        color: Color,
    ) {
        self.add(a, tc_a, color);
        self.add(b, tc_b, color);
        self.add(c, tc_c, color);
    }

    pub fn add_tile(
        &mut self,
        position: [f32; 2],
        size: [f32; 2],
        tc: [f32; 2],
        tc_size: [f32; 2],
        color: Color,
    ) {
        let [c00, c10, c01, c11] = get_corners(position, size);
        let [tc00, tc10, tc01, tc11] = get_corners(tc, tc_size);

        self.add_triangle(c00, c10, c11, tc00, tc10, tc11, color);
        self.add_triangle(c00, c11, c01, tc00, tc11, tc01, color);
    }

    fn add(&mut self, position: [f32; 2], tc: [f32; 2], color: Color) {
        self.vertices.push(TexturedVertex {
            position,
            tc,
            color: color.to_array(),
        });
    }

    pub fn get(&self) -> &Vec<TexturedVertex> {
        &self.vertices
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::testing::*;

    const TC_SIZE: [f32; 2] = [0.25, 0.35];
    const TC00: [f32; 2] = [0.1, 0.2];
    const TC10: [f32; 2] = [0.35, 0.2];
    const TC01: [f32; 2] = [0.1, 0.55];
    const TC11: [f32; 2] = [0.35, 0.55];
    pub const TC: [[f32; 2]; 4] = [TC00, TC10, TC01, TC11];

    #[test]
    fn test_add_tile() {
        let mut builder = TexturedTriangleBuilder::default();

        builder.add_tile(P00, SIZE, TC00, TC_SIZE, COLOR);

        assert_textured_tile(builder.get(), POS, TC, COLOR);
    }

    #[test]
    fn test_add_clear() {
        let mut builder = TexturedTriangleBuilder::default();

        builder.add_tile(P11, SIZE, TC11, TC_SIZE, COLOR);
        builder.clear();

        assert!(builder.get().is_empty());
    }
}
