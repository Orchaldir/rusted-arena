extern crate rusted_tiles;

use rusted_tiles::rendering::glium::window::GliumWindow;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::{App, Renderer, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MapApp {
    _map: Vec<u8>,
    tile_renderer: TileRenderer,
}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        self.tile_renderer
            .add_text([0, 0], "Test!", [1.0, 0.0, 0.0]);

        renderer.start([0.0, 0.0, 0.0]);
        self.tile_renderer.render(renderer);
        renderer.finish();
    }
}

fn main() {
    let mut window = GliumWindow::new("Map Example", [40, 20], [16, 16]);
    let app = Rc::new(RefCell::new(MapApp {
        _map: Vec::new(),
        tile_renderer: window.get_tile_renderer(),
    }));

    window.run(app.clone());
}
