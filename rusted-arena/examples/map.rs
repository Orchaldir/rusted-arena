extern crate rusted_tiles;

use rusted_tiles::rendering::colored::ColoredTriangleBuilder;
use rusted_tiles::rendering::glium::window::GliumWindow;
use rusted_tiles::rendering::{App, Renderer, Window};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct MapApp {
    _map: Vec<u8>,
}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        let mut builder = ColoredTriangleBuilder::default();

        builder.add_triangle([-0.5, -0.5], [0.0, 0.5], [0.5, -0.25], [0.0, 1.0, 0.0]);
        builder.add_tile([-1.0, -1.0], [0.5, 0.5], [1.0, 0.0, 0.0]);
        builder.add_polygon(
            &[[0.5, -1.0], [1.0, -1.0], [0.8, 0.8], [0.5, 0.1]],
            [1.0, 1.0, 0.0],
        );

        renderer.start([0.0, 0.0, 1.0]);
        renderer.render_colored(builder.get());
        renderer.finish();
    }
}

fn main() {
    let app = Rc::new(RefCell::new(MapApp::default()));
    let mut window = GliumWindow::new("Map Example", [40, 20], [16, 16]);

    window.run(app.clone());
}
