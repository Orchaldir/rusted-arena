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
        color: [f32; 3],
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
        color: [f32; 3],
    ) {
        let [c00, c10, c01, c11] = get_corners(position, size);
        let [tc00, tc10, tc01, tc11] = get_corners(tc, tc_size);

        self.add_triangle(c00, c10, c11, tc00, tc10, tc11, color);
        self.add_triangle(c00, c11, c01, tc00, tc11, tc01, color);
    }

    fn add(&mut self, position: [f32; 2], tc: [f32; 2], color: [f32; 3]) {
        self.vertices.push(TexturedVertex {
            position,
            tc,
            color,
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

    const SIZE: [f32; 2] = [0.5, 1.5];
    const P00: [f32; 2] = [2.0, 4.0];
    const P10: [f32; 2] = [2.5, 4.0];
    const P01: [f32; 2] = [2.0, 5.5];
    const P11: [f32; 2] = [2.5, 5.5];

    const TC_SIZE: [f32; 2] = [0.25, 0.35];
    const TC00: [f32; 2] = [0.1, 0.2];
    const TC10: [f32; 2] = [0.35, 0.2];
    const TC01: [f32; 2] = [0.1, 0.55];
    const TC11: [f32; 2] = [0.35, 0.55];

    const COLOR: [f32; 3] = [0.1, 0.2, 0.3];

    #[test]
    fn test_add_tile() {
        let mut builder = TexturedTriangleBuilder::default();

        builder.add_tile(P00, SIZE, TC00, TC_SIZE, COLOR);

        let vertices = builder.get();

        assert_eq!(vertices.len(), 6);
        assert_tile(&vertices[0], P00, TC00);
        assert_tile(&vertices[1], P10, TC10);
        assert_tile(&vertices[2], P11, TC11);
        assert_tile(&vertices[3], P00, TC00);
        assert_tile(&vertices[4], P11, TC11);
        assert_tile(&vertices[5], P01, TC01);
    }

    fn assert_tile(vertex: &TexturedVertex, pos: [f32; 2], tc: [f32; 2]) {
        assert_eq!(vertex.position, pos);
        assert_eq!(vertex.tc, tc);
        assert_eq!(vertex.color, COLOR);
    }
}
