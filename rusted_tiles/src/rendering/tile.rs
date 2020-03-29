use super::ascii::AsciiBuilder;
use super::colored::ColoredTriangleBuilder;
use super::Renderer;
use crate::math::color::*;
use crate::math::point::Point;

pub trait TileRenderer {
    fn add_tile(&mut self, index: Point, color: Color);
    fn add_polygon(&mut self, index: Point, corners: &[[f32; 2]], color: Color);

    fn add_ascii(&mut self, index: Point, ascii: u8, color: Color);
    fn add_big_ascii(&mut self, index: Point, size: u32, ascii: u8, color: Color);

    fn add_text(&mut self, index: Point, string: &str, color: Color);
    fn add_big_text(&mut self, index: Point, size: u32, string: &str, color: Color);

    fn clear(&mut self);
    fn render(&self, renderer: &mut dyn Renderer);
}

pub struct TileRendererToWindow {
    ascii_builder: AsciiBuilder,
    colored_builder: ColoredTriangleBuilder,
    start: Point,
    tile_size: Point,
    tile_pixel: [f32; 2],
}

impl TileRendererToWindow {
    pub fn new(start: Point, tile_size: Point) -> TileRendererToWindow {
        let tile_pixel = [tile_size.x as f32, tile_size.y as f32];

        TileRendererToWindow {
            ascii_builder: AsciiBuilder::default(),
            colored_builder: ColoredTriangleBuilder::default(),
            start,
            tile_size,
            tile_pixel,
        }
    }

    fn calculate_position(&self, index: Point) -> [f32; 2] {
        let pos = self.start + index * self.tile_size;
        [pos.x as f32, pos.y as f32]
    }

    fn calculate_tile_size(&self, size: u32) -> [f32; 2] {
        let size = size as f32;
        [self.tile_pixel[0] * size, self.tile_pixel[1] * size]
    }
}

impl TileRenderer for TileRendererToWindow {
    fn add_tile(&mut self, index: Point, color: Color) {
        let position = self.calculate_position(index);

        self.colored_builder
            .add_tile(position, self.tile_pixel, color);
    }

    fn add_polygon(&mut self, index: Point, corners: &[[f32; 2]], color: Color) {
        let pos = self.calculate_position(index);
        let polygon: Vec<[f32; 2]> = corners
            .iter()
            .map(|[x, y]| {
                [
                    pos[0] + x * self.tile_pixel[0],
                    pos[1] + y * self.tile_pixel[1],
                ]
            })
            .collect();

        self.colored_builder.add_polygon(&polygon, color);
    }

    fn add_ascii(&mut self, index: Point, ascii: u8, color: Color) {
        let position = self.calculate_position(index);

        self.ascii_builder
            .add_u8(position, self.tile_pixel, ascii, color);
    }

    fn add_big_ascii(&mut self, index: Point, size: u32, ascii: u8, color: Color) {
        let position = self.calculate_position(index);
        let tile_size = self.calculate_tile_size(size);

        self.ascii_builder.add_u8(position, tile_size, ascii, color);
    }

    fn add_text(&mut self, index: Point, string: &str, color: Color) {
        let position = self.calculate_position(index);

        self.ascii_builder
            .add_string(position, self.tile_pixel, string, color);
    }

    fn add_big_text(&mut self, index: Point, size: u32, string: &str, color: Color) {
        let position = self.calculate_position(index);
        let tile_size = self.calculate_tile_size(size);

        self.ascii_builder
            .add_string(position, tile_size, string, color);
    }

    fn clear(&mut self) {
        self.colored_builder.clear();
        self.ascii_builder.clear();
    }

    fn render(&self, renderer: &mut dyn Renderer) {
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

    impl TileRendererToWindow {
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
    const COLOR: Color = Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
    };

    const INDEX0: Point = Point { x: 3, y: 4 };
    const INDEX1: Point = Point { x: 5, y: 10 };
    const INDEX2: Point = Point { x: 6, y: 0 };
    const START: Point = Point { x: 100, y: 200 };
    const TILE_SIZE: Point = Point { x: 10, y: 20 };

    #[test]
    fn test_add_tile() {
        let mut renderer = TileRendererToWindow::new(START, TILE_SIZE);

        renderer.add_tile(INDEX0, COLOR);

        assert_tile(renderer.get_colored(), POS, COLOR);
    }

    #[test]
    fn test_add_polygon() {
        let mut renderer = TileRendererToWindow::new(START, TILE_SIZE);

        renderer.add_polygon(INDEX0, &[[0.0, 0.0], [1.0, 0.0], [0.5, 1.0]], COLOR);

        let vertices = renderer.get_colored();

        assert_eq!(vertices.len(), 3);
        assert_colored(&vertices[0], P00, COLOR);
        assert_colored(&vertices[1], P10, COLOR);
        assert_colored(&vertices[2], [135.0, 300.0], COLOR);
    }

    #[test]
    fn test_add_ascii() {
        let mut renderer = TileRendererToWindow::new(START, TILE_SIZE);

        renderer.add_ascii(INDEX0, b'A', COLOR);

        assert_textured_tile(renderer.get_textured(), POS, TC_A, COLOR);
    }

    #[test]
    fn test_add_big_ascii() {
        let b00 = [150.0, 400.0];
        let b10 = [170.0, 400.0];
        let b01 = [150.0, 440.0];
        let b11 = [170.0, 440.0];
        let big = [b00, b10, b01, b11];

        let mut renderer = TileRendererToWindow::new(START, TILE_SIZE);

        renderer.add_big_ascii(INDEX1, 2, b'?', COLOR);

        assert_textured_tile(renderer.get_textured(), big, TC_Q, COLOR);
    }

    #[test]
    fn test_clear() {
        let mut renderer = TileRendererToWindow::new(START, TILE_SIZE);

        renderer.add_tile(INDEX0, COLOR);
        renderer.add_ascii(INDEX2, b'P', COLOR);
        renderer.clear();

        assert!(renderer.get_colored().is_empty());
        assert!(renderer.get_textured().is_empty());
    }
}
