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
}
