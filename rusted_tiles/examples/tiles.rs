extern crate rusted_tiles;

use rusted_tiles::math::color::*;
use rusted_tiles::math::point::Point;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MapApp {
    tile_renderer: TileRenderer,
}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        self.tile_renderer.add_tile(Point { x: 0, y: 0 }, RED);
        self.tile_renderer.add_polygon(
            Point { x: 30, y: 15 },
            &[[0.0, 0.2], [1.0, 0.2], [0.5, 1.0]],
            RED,
        );
        self.tile_renderer
            .add_ascii(Point { x: 1, y: 0 }, b'@', WHITE);
        self.tile_renderer
            .add_big_ascii(Point { x: 5, y: 10 }, 3, b'D', WHITE);
        self.tile_renderer
            .add_text(Point { x: 10, y: 15 }, "Hello", CYAN);
        self.tile_renderer
            .add_big_text(Point { x: 15, y: 5 }, 5, "Big", YELLOW);

        renderer.start(BLACK);
        self.tile_renderer.render(renderer);
        renderer.finish();
    }

    fn on_button_released(&mut self, _: Point, _: MouseButton) {}
    fn on_key_released(&mut self, _: VirtualKeyCode) {}
}

fn main() {
    let mut window = GliumWindow::new(
        "Example with tiles",
        Point { x: 40, y: 20 },
        Point { x: 16, y: 16 },
    );
    let app = Rc::new(RefCell::new(MapApp {
        tile_renderer: window.get_tile_renderer(),
    }));

    window.run(app.clone());
}
