use crate::math::get_corners;

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
        let [corner00, corner10, corner01, corner11] = get_corners(position, size);

        self.add_triangle(corner00, corner10, corner11, color);
        self.add_triangle(corner00, corner11, corner01, color);
    }

    pub fn add_polygon(&mut self, corners: &[[f32; 2]], color: [f32; 3]) {
        if corners.len() < 3 {
            panic!("Polygon must have 3 corners!");
        }

        let last_start_index = corners.len() - 1;

        for i in 1..last_start_index {
            self.add_triangle(corners[0], corners[i], corners[i + 1], color);
        }
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
    use crate::rendering::testing::*;

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

        assert_tile(builder.get(), POS, COLOR);
    }
}
