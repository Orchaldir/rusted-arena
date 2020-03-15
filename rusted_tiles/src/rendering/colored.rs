#[derive(Copy, Clone)]
pub struct ColoredVertex {
    pub position: [f32; 2],
    pub color: [f32; 3],
}

#[derive(Default)]
pub struct ColoredTriangleBuilder {
    vertices: Vec<ColoredVertex>,
}

impl ColoredTriangleBuilder {
    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn add_triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2], color: [f32; 3]) {
        self.add(a, color);
        self.add(b, color);
        self.add(c, color);
    }

    pub fn add_tile(&mut self, position: [f32; 2], size: [f32; 2], color: [f32; 3]) {
        let corner00 = position;
        let corner10 = [position[0] + size[0], position[1]];
        let corner01 = [position[0], position[1] + size[1]];
        let corner11 = [position[0] + size[0], position[1] + size[1]];

        self.add_triangle(corner00, corner10, corner11, color);
        self.add_triangle(corner00, corner11, corner01, color);
    }

    fn add(&mut self, position: [f32; 2], color: [f32; 3]) {
        self.vertices.push(ColoredVertex { position, color });
    }

    pub fn get(&self) -> &Vec<ColoredVertex> {
        &self.vertices
    }
}
