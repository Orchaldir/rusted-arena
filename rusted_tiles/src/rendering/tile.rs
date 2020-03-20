use super::ascii::AsciiBuilder;
use super::colored::ColoredTriangleBuilder;
use super::Renderer;

pub struct TileRenderer {
    ascii_builder: AsciiBuilder,
    colored_builder: ColoredTriangleBuilder,
    start: [f32; 2],
    tile_size: [f32; 2],
}

impl TileRenderer {
    pub fn new(start: [f32; 2], tile_size: [f32; 2]) -> TileRenderer {
        TileRenderer {
            ascii_builder: AsciiBuilder::default(),
            colored_builder: ColoredTriangleBuilder::default(),
            start,
            tile_size,
        }
    }

    pub fn add_tile(&mut self, index: [u32; 2], color: [f32; 3]) {
        let position = self.calculate_position(index);

        self.colored_builder
            .add_tile(position, self.tile_size, color);
    }

    pub fn add_polygon(&mut self, index: [u32; 2], corners: &[[f32; 2]], color: [f32; 3]) {
        let pos = self.calculate_position(index);
        let polygon: Vec<[f32; 2]> = corners
            .iter()
            .map(|[x, y]| {
                [
                    pos[0] + x * self.tile_size[0],
                    pos[1] + y * self.tile_size[1],
                ]
            })
            .collect();

        self.colored_builder.add_polygon(&polygon, color);
    }

    pub fn add_ascii(&mut self, index: [u32; 2], ascii: u8, color: [f32; 3]) {
        let position = self.calculate_position(index);

        self.ascii_builder
            .add_u8(position, self.tile_size, ascii, color);
    }

    pub fn add_big_ascii(&mut self, index: [u32; 2], size: u32, ascii: u8, color: [f32; 3]) {
        let position = self.calculate_position(index);
        let tile_size = self.calculate_tile_size(size);

        self.ascii_builder.add_u8(position, tile_size, ascii, color);
    }

    pub fn add_text(&mut self, index: [u32; 2], string: &str, color: [f32; 3]) {
        let position = self.calculate_position(index);

        self.ascii_builder
            .add_string(position, self.tile_size, string, color);
    }

    pub fn add_big_text(&mut self, index: [u32; 2], size: u32, string: &str, color: [f32; 3]) {
        let position = self.calculate_position(index);
        let tile_size = self.calculate_tile_size(size);

        self.ascii_builder
            .add_string(position, tile_size, string, color);
    }

    fn calculate_position(&self, position: [u32; 2]) -> [f32; 2] {
        let x = self.start[0] + position[0] as f32 * self.tile_size[0];
        let y = self.start[1] + position[1] as f32 * self.tile_size[1];
        [x, y]
    }

    fn calculate_tile_size(&self, size: u32) -> [f32; 2] {
        let size = size as f32;
        [self.tile_size[0] * size, self.tile_size[1] * size]
    }

    pub fn render<R: Renderer>(&self, renderer: &mut R) {
        renderer.render_colored(self.colored_builder.get());
        renderer.render_textured(self.ascii_builder.get());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rendering::colored::ColoredVertex;
    use crate::rendering::testing::*;
    use crate::rendering::textured::TexturedVertex;

    impl TileRenderer {
        pub fn get_colored(&self) -> &Vec<ColoredVertex> {
            &self.colored_builder.get()
        }

        pub fn get_textured(&self) -> &Vec<TexturedVertex> {
            &self.ascii_builder.get()
        }
    }

    const P00: [f32; 2] = [130.0, 280.0];
    const P10: [f32; 2] = [140.0, 280.0];
    const P01: [f32; 2] = [130.0, 300.0];
    const P11: [f32; 2] = [140.0, 300.0];
    const POS: [[f32; 2]; 4] = [P00, P10, P01, P11];
    const COLOR: [f32; 3] = [0.1, 0.2, 0.3];

    #[test]
    fn test_add_tile() {
        let mut renderer = TileRenderer::new([100.0, 200.0], [10.0, 20.0]);

        renderer.add_tile([3, 4], COLOR);

        assert_tile(renderer.get_colored(), POS, COLOR);
    }

    #[test]
    fn test_add_polygon() {
        let mut renderer = TileRenderer::new([100.0, 200.0], [10.0, 20.0]);

        renderer.add_polygon([3, 4], &[[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]], COLOR);

        let vertices = renderer.get_colored();

        assert_eq!(vertices.len(), 3);
        assert_colored(&vertices[0], P00, COLOR);
        assert_colored(&vertices[1], P10, COLOR);
        assert_colored(&vertices[2], [135.0, 300.0], COLOR);
    }

    #[test]
    fn test_add_ascii() {
        let mut renderer = TileRenderer::new([100.0, 200.0], [10.0, 20.0]);

        renderer.add_ascii([3, 4], b'A', COLOR);

        assert_u8(renderer.get_textured(), POS, TC_A, COLOR);
    }

    #[test]
    fn test_add_big_ascii() {
        let b00 = [150.0, 400.0];
        let b10 = [170.0, 400.0];
        let b01 = [150.0, 440.0];
        let b11 = [170.0, 440.0];
        let big = [b00, b10, b01, b11];

        let mut renderer = TileRenderer::new([100.0, 200.0], [10.0, 20.0]);

        renderer.add_big_ascii([5, 10], 2, b'?', COLOR);

        assert_u8(renderer.get_textured(), big, TC_Q, COLOR);
    }
}
