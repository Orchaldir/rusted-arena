pub mod ascii;
pub mod colored;
pub mod glium;
pub mod textured;

pub trait Renderer {
    fn start(&mut self, color: [f32; 3]);
    fn render_colored(&mut self, vertices: &Vec<colored::ColoredVertex>);
    fn finish(&mut self);
}
