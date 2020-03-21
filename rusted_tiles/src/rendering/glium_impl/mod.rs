pub mod shader;
pub mod texture;
pub mod window;

use super::Renderer;
use crate::math::color::Color;
use crate::rendering::colored::ColoredVertex;
use crate::rendering::textured::TexturedVertex;
use cgmath::ortho;
use glium::{Program, Surface};

const INDICES: glium::index::NoIndices =
    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

pub struct GliumRenderer {
    display: glium::Display,
    target: Option<glium::Frame>,
    colored_program: Program,
    textured_program: Program,
    texture: glium::texture::Texture2d,
    matrix: cgmath::Matrix4<f32>,
}

impl GliumRenderer {
    pub fn new(display: glium::Display, size: [u32; 2]) -> GliumRenderer {
        let colored_program = shader::load_program(&display, "colored.vertex", "colored.fragment");
        let textured_program =
            shader::load_program(&display, "textured.vertex", "textured.fragment");

        let texture = texture::load_texture(&display, "ascii.png").unwrap();

        let matrix: cgmath::Matrix4<f32> =
            ortho(0.0, size[0] as f32, 0.0, size[1] as f32, -1.0, 1.0);

        GliumRenderer {
            display,
            target: None,
            colored_program,
            textured_program,
            texture,
            matrix,
        }
    }
}

impl Renderer for GliumRenderer {
    fn start(&mut self, color: Color) {
        let mut target = self.display.draw();
        target.clear_color(color.r, color.g, color.b, 1.0);
        self.target = Some(target);
    }

    fn render_colored(&mut self, vertices: &[ColoredVertex]) {
        let target = self.target.as_mut().unwrap();
        let vertex_buffer = glium::VertexBuffer::new(&self.display, vertices).unwrap();

        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(self.matrix),
            tex: &self.texture,
        };

        target
            .draw(
                &vertex_buffer,
                &INDICES,
                &self.colored_program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
    }

    fn render_textured(&mut self, vertices: &[TexturedVertex]) {
        let target = self.target.as_mut().unwrap();
        let vertex_buffer = glium::VertexBuffer::new(&self.display, vertices).unwrap();

        let uniforms = uniform! {
            matrix: Into::<[[f32; 4]; 4]>::into(self.matrix),
            tex: &self.texture,
        };

        target
            .draw(
                &vertex_buffer,
                &INDICES,
                &self.textured_program,
                &uniforms,
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
