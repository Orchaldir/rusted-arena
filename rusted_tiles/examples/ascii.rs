extern crate rusted_tiles;

use rusted_tiles::math::color::*;
use rusted_tiles::rendering::ascii::AsciiBuilder;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct MapApp {}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        let mut builder = AsciiBuilder::default();

        builder.add_u8([200.0, 200.0], [100.0, 100.0], b'a', RED);
        builder.add_char([300.0, 200.0], [100.0, 100.0], 'b', GREEN);
        builder.add_string([300.0, 500.0], [20.0, 20.0], "Test?", WHITE);
        builder.add_string(
            [0.0, 50.0],
            [20.0, 20.0],
            "Non-Ascii Symbols are replaced with '🎉'!",
            YELLOW,
        );

        renderer.start(BLUE);
        renderer.render_textured(builder.get());
        renderer.finish();
    }

    fn on_button_released(&mut self, _: [u32; 2], _: MouseButton) {}
    fn on_key_released(&mut self, _: VirtualKeyCode) {}
}

fn main() {
    let mut window = GliumWindow::new("Example with ascii", [80, 60], [10, 10]);
    let app = Rc::new(RefCell::new(MapApp::default()));

    window.run(app.clone());
}
