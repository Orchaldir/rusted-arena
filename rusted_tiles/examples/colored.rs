extern crate rusted_tiles;

use rusted_tiles::math::color::*;
use rusted_tiles::rendering::colored::ColoredTriangleBuilder;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct MapApp {}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        let mut builder = ColoredTriangleBuilder::default();

        builder.add_triangle([-0.5, -0.5], [0.0, 0.5], [0.5, -0.25], GREEN);
        builder.add_tile([-1.0, -1.0], [0.5, 0.5], RED);
        builder.add_polygon(&[[0.5, -1.0], [1.0, -1.0], [0.8, 0.8], [0.5, 0.1]], YELLOW);

        renderer.start(BLUE);
        renderer.render_colored(builder.get());
        renderer.finish();
    }

    fn on_button_released(&mut self, _: [u32; 2], _: MouseButton) {}
    fn on_key_released(&mut self, _: VirtualKeyCode) {}
}

fn main() {
    let mut window = GliumWindow::new("Example with colored Triangles", [80, 60], [10, 10]);
    let app = Rc::new(RefCell::new(MapApp::default()));

    window.run(app.clone());
}
