extern crate rusted_tiles;

use rusted_tiles::math::color::*;
use rusted_tiles::math::point::Point;
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

        builder.add_triangle([400.0, 300.0], [600.0, 300.0], [500.0, 400.0], GREEN);
        builder.add_tile([100.0, 100.0], [100.0, 100.0], RED);
        builder.add_polygon(
            &[
                [200.0, 300.0],
                [200.0, 400.0],
                [250.0, 500.0],
                [300.0, 400.0],
            ],
            YELLOW,
        );

        renderer.start(BLUE);
        renderer.render_colored(builder.get());
        renderer.finish();
    }

    fn on_button_released(&mut self, _: Point, _: MouseButton) {}
    fn on_key_released(&mut self, _: VirtualKeyCode) {}
}

fn main() {
    let mut window = GliumWindow::default_size("Example with colored Triangles");
    let app = Rc::new(RefCell::new(MapApp::default()));

    window.run(app.clone());
}
