#[macro_use]
extern crate glium;
extern crate image;
extern crate rusted_tiles;

use image::io::Reader;
use rusted_tiles::rendering::shader::load_program;
use rusted_tiles::rendering::textured::TexturedTriangleBuilder;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Example with textured Triangles");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let image = Reader::open("resources/image/ascii.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba();
    let image_dimensions = image.dimensions();
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let mut builder = TexturedTriangleBuilder::default();

    builder.add_tile(
        [-0.5, -0.5],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 1.0],
        [1.0, 0.0, 0.0],
    );

    let vertex_buffer = glium::VertexBuffer::new(&display, builder.get()).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program = load_program(&display, "textured.vertex", "textured.fragment");

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

        let uniforms = uniform! {
            tex: &texture,
        };

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}
