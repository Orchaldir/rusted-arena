pub mod ascii;
pub mod colored;
pub mod glium_impl;
pub mod testing;
pub mod textured;
pub mod tile;

use std::cell::RefCell;
use std::rc::Rc;

pub trait Renderer {
    fn start(&mut self, color: [f32; 3]);
    fn render_colored(&mut self, vertices: &[colored::ColoredVertex]);
    fn render_textured(&mut self, vertices: &[textured::TexturedVertex]);
    fn finish(&mut self);
}

pub trait App {
    fn render(&mut self, renderer: &mut dyn Renderer);
}

pub trait Window {
    fn get_tile_renderer(&self) -> tile::TileRenderer;
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> !;
}
