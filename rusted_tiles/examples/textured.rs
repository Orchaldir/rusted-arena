extern crate rusted_tiles;

use rusted_tiles::math::color::*;
use rusted_tiles::math::point::Point;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::textured::TexturedTriangleBuilder;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct MapApp {}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        let mut builder = TexturedTriangleBuilder::default();

        builder.add_tile([200.0, 100.0], [400.0, 400.0], [0.0, 0.0], [1.0, 1.0], RED);

        renderer.start(BLACK);
        renderer.render_textured(builder.get());
        renderer.finish();
    }

    fn on_button_released(&mut self, _: Point, _: MouseButton) {}
    fn on_key_released(&mut self, _: VirtualKeyCode) {}
}

fn main() {
    let mut window = GliumWindow::default_size("Example with textured Triangles");
    let app = Rc::new(RefCell::new(MapApp::default()));

    window.run(app.clone());
}
