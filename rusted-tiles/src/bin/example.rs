#[macro_use]
extern crate glium;

#[derive(Copy, Clone)]
struct ColoredVertex {
    position: [f32; 2],
    color: [f32; 3],
}

implement_vertex!(ColoredVertex, position, color);

struct ColoredTriangleBuilder {
    vertices: Vec<ColoredVertex>,
}

impl ColoredTriangleBuilder {
    pub fn new() -> ColoredTriangleBuilder {
        ColoredTriangleBuilder {
            vertices: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.vertices.clear();
    }

    pub fn add_triangle(&mut self, a: [f32; 2], b: [f32; 2], c: [f32; 2], color: [f32; 3]) {
        self.add(a, color);
        self.add(b, color);
        self.add(c, color);
    }

    fn add(&mut self, position: [f32; 2], color: [f32; 3]) {
        self.vertices.push(ColoredVertex { position, color });
    }

    pub fn get(&self) -> &Vec<ColoredVertex> {
        &self.vertices
    }
}

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut builder = ColoredTriangleBuilder::new();

    builder.clear();
    builder.add_triangle([-0.5, -0.5], [0.0, 0.5], [0.5, -0.25], [0.0, 1.0, 0.0]);

    let vertex_buffer = glium::VertexBuffer::new(&display, builder.get()).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 v_color;

        void main() {
            v_color = color;
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 v_color;
        out vec4 color;

        void main() {
            color = vec4(v_color, 1.0);
        }
    "#;

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::RedrawRequested(_) => (),
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
