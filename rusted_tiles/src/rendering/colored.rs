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

    pub fn clear(&mut self) {
        self.vertices.clear();
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
    #[should_panic]
    fn test_add_polygon_with_no_corners() {
        ColoredTriangleBuilder::default().add_polygon(&[], COLOR);
    }

    #[test]
    #[should_panic]
    fn test_add_polygon_with_one_corner() {
        ColoredTriangleBuilder::default().add_polygon(&[[0.0, 0.0]], COLOR);
    }

    #[test]
    #[should_panic]
    fn test_add_polygon_with_two_corners() {
        ColoredTriangleBuilder::default().add_polygon(&[[0.0, 0.0], [1.0, 0.0]], COLOR);
    }

    #[test]
    fn test_add_polygon() {
        let mut builder = ColoredTriangleBuilder::default();

        let p = [2.3, 6.5];

        builder.add_polygon(&[P00, P10, P11, p, P01], COLOR);

        let vertices = builder.get();

        assert_eq!(vertices.len(), 9);
        assert_colored(&vertices[0], P00, COLOR);
        assert_colored(&vertices[1], P10, COLOR);
        assert_colored(&vertices[2], P11, COLOR);
        assert_colored(&vertices[3], P00, COLOR);
        assert_colored(&vertices[4], P11, COLOR);
        assert_colored(&vertices[5], p, COLOR);
        assert_colored(&vertices[6], P00, COLOR);
        assert_colored(&vertices[7], p, COLOR);
        assert_colored(&vertices[8], P01, COLOR);
    }

    #[test]
    fn test_add_tile() {
        let mut builder = ColoredTriangleBuilder::default();

        builder.add_tile(P00, SIZE, COLOR);

        assert_tile(builder.get(), POS, COLOR);
    }
}
