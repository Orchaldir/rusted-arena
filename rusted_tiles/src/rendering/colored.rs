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

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: [f32; 2] = [0.5, 1.5];
    const P00: [f32; 2] = [2.0, 4.0];
    const P10: [f32; 2] = [2.5, 4.0];
    const P01: [f32; 2] = [2.0, 5.5];
    const P11: [f32; 2] = [2.5, 5.5];
    const COLOR: [f32; 3] = [0.1, 0.2, 0.3];

    #[test]
    fn test_add_triangle() {
        let mut builder = ColoredTriangleBuilder::default();

        builder.add_triangle(P00, P10, P11, COLOR);

        let vertices = builder.get();

        assert_eq!(vertices.len(), 3);
        assert_eq!(vertices[0].position, P00);
        assert_eq!(vertices[1].position, P10);
        assert_eq!(vertices[2].position, P11);

        for vertex in vertices {
            assert_eq!(vertex.color, COLOR);
        }
    }

    #[test]
    fn test_add_tile() {
        let mut builder = ColoredTriangleBuilder::default();

        builder.add_tile(P00, SIZE, COLOR);

        let vertices = builder.get();

        assert_eq!(vertices.len(), 6);
        assert_eq!(vertices[0].position, P00);
        assert_eq!(vertices[1].position, P10);
        assert_eq!(vertices[2].position, P11);
        assert_eq!(vertices[3].position, P00);
        assert_eq!(vertices[4].position, P11);
        assert_eq!(vertices[5].position, P01);

        for vertex in vertices {
            assert_eq!(vertex.color, COLOR);
        }
    }
}
