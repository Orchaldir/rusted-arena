use super::colored::ColoredTriangleBuilder;
use super::Renderer;

pub struct TileRenderer {
    colored_builder: ColoredTriangleBuilder,
    start: [f32; 2],
    tile_size: [f32; 2],
}

impl TileRenderer {
    pub fn new(start: [f32; 2], tile_size: [f32; 2]) -> TileRenderer {
        TileRenderer {
            colored_builder: ColoredTriangleBuilder::default(),
            start,
            tile_size,
        }
    }

    pub fn add_tile(&mut self, position: [u32; 2], color: [f32; 3]) {
        let x = self.start[0] + position[0] as f32 * self.tile_size[0];
        let y = self.start[1] + position[1] as f32 * self.tile_size[1];

        self.colored_builder.add_tile([x, y], self.tile_size, color);
    }

    pub fn render<R: Renderer>(&self, renderer: &mut R) {
        renderer.render_colored(self.colored_builder.get());
    }
}
