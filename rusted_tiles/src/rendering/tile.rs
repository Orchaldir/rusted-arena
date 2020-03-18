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

        self.colored_builder.add_tile(position, self.tile_size, color);
    }

    pub fn add_ascii(&mut self, index: [u32; 2], ascii: u8, color: [f32; 3]) {
        let position = self.calculate_position(index);

        self.ascii_builder.add_u8(position, self.tile_size, ascii, color);
    }

    fn calculate_position(&mut self, position: [u32; 2]) -> [f32; 2] {
        let x = self.start[0] + position[0] as f32 * self.tile_size[0];
        let y = self.start[1] + position[1] as f32 * self.tile_size[1];
        [x, y]
    }

    pub fn render<R: Renderer>(&self, renderer: &mut R) {
        renderer.render_colored(self.colored_builder.get());
        renderer.render_textured(self.ascii_builder.get());
    }
}
