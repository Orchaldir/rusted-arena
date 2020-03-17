pub mod shader;
pub mod texture;

use super::Renderer;
use crate::rendering::colored::ColoredVertex;
use glium::{Program, Surface};

const INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

pub struct GliumRenderer {
    display: glium::Display,
    program: Program,
    target: Option<glium::Frame>,
}

impl GliumRenderer {
    pub fn new(display: glium::Display) -> GliumRenderer {
        let program = shader::load_program(&display, "colored.vertex", "colored.fragment");
        GliumRenderer {
            display,
            program,
            target: None,
        }
    }
}

impl Renderer for GliumRenderer {
    fn start(&mut self, color: [f32; 3]) {
        let mut target = self.display.draw();
        target.clear_color(color[0], color[1], color[2], 1.0);
        self.target = Some(target);
    }

    fn render_colored(&mut self, vertices: &Vec<ColoredVertex>) {
        let target = self.target.as_mut().unwrap();
        let vertex_buffer = glium::VertexBuffer::new(&self.display, vertices).unwrap();

        target
            .draw(
                &vertex_buffer,
                &INDICES,
                &self.program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
    }

    fn finish(&mut self) {
        if let Some(target) = self.target.take() {
            target.finish().unwrap();
        }
    }
}
