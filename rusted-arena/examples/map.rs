extern crate rusted_tiles;

use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::{App, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MapApp {
    pos: [u32; 2],
    tile_renderer: TileRenderer,
}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        self.tile_renderer.clear();
        self.tile_renderer
            .add_text([0, 0], "Test!", [1.0, 0.0, 0.0]);
        self.tile_renderer
            .add_ascii(self.pos, b'a', [1.0, 0.0, 0.0]);

        renderer.start([0.0, 0.0, 0.0]);
        self.tile_renderer.render(renderer);
        renderer.finish();
    }

    fn on_key_released(&mut self, key: VirtualKeyCode) {
        println!("Key '{:?}' released", key);
        match key {
            VirtualKeyCode::Down => self.pos[1] -= 1,
            VirtualKeyCode::Left => self.pos[0] -= 1,
            VirtualKeyCode::Right => self.pos[0] += 1,
            VirtualKeyCode::Up => self.pos[1] += 1,
            _ => (),
        }
    }
}

fn main() {
    let mut window = GliumWindow::new("Map Example", [40, 20], [16, 16]);
    let app = Rc::new(RefCell::new(MapApp {
        pos: [10, 10],
        tile_renderer: window.get_tile_renderer(),
    }));

    window.run(app.clone());
}
