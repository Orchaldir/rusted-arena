extern crate glium;
extern crate rusted_tiles;

use glium::backend::Facade;
use rusted_tiles::rendering::colored::ColoredTriangleBuilder;
use std::fs;

fn load_program<F: Facade>(display: &F, vertex_file: &str, fragment_file: &str) -> glium::Program {
    let path = "resources/shader/";
    let vertex_shader =
        fs::read_to_string([path, vertex_file].concat()).expect("Could not load vertex shader");
    let fragment_shader =
        fs::read_to_string([path, fragment_file].concat()).expect("Could not load vertex shader");

    glium::Program::from_source(display, &vertex_shader, &fragment_shader, None).unwrap()
}

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut builder = ColoredTriangleBuilder::default();

    builder.add_triangle([-0.5, -0.5], [0.0, 0.5], [0.5, -0.25], [0.0, 1.0, 0.0]);
    builder.add_tile([-1.0, -1.0], [0.5, 0.5], [1.0, 0.0, 0.0]);

    let vertex_buffer = glium::VertexBuffer::new(&display, builder.get()).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = load_program(&display, "colored.vertex", "colored.fragment");

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
