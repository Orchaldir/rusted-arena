pub mod ascii;
pub mod colored;
pub mod glium_impl;
pub mod testing;
pub mod textured;
pub mod tile;

use crate::math::color::Color;
pub use glium::glutin::event::{MouseButton, VirtualKeyCode};
use std::cell::RefCell;
use std::rc::Rc;

pub trait Renderer {
    fn start(&mut self, color: Color);
    fn render_colored(&mut self, vertices: &[colored::ColoredVertex]);
    fn render_textured(&mut self, vertices: &[textured::TexturedVertex]);
    fn finish(&mut self);
}

pub trait App {
    fn render(&mut self, renderer: &mut dyn Renderer);
    fn on_button_released(&mut self, position: [u32; 2], button: MouseButton);
    fn on_key_released(&mut self, key: VirtualKeyCode);
}

pub trait Window {
    fn get_tile_renderer(&self) -> tile::TileRenderer;
    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> !;
}
