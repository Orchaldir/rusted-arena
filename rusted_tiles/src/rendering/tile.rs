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
    fn test_add_ascii() {
        let mut renderer = TileRenderer::new([100.0, 200.0], [10.0, 20.0]);

        renderer.add_ascii([3, 4], b'A', COLOR);

        assert_u8(renderer.get_textured(), POS, TC_A, COLOR);
    }
}
