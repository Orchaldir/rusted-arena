extern crate glium;
extern crate rusted_tiles;

use rusted_tiles::math::color::*;
use rusted_tiles::rendering::glium_impl::GliumRenderer;
use rusted_tiles::rendering::textured::TexturedTriangleBuilder;
use rusted_tiles::rendering::Renderer;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Example with textured Triangles");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut builder = TexturedTriangleBuilder::default();

    builder.add_tile([-0.5, -0.5], [1.0, 1.0], [0.0, 0.0], [1.0, 1.0], RED);

    let mut render = GliumRenderer::new(display);

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

        render.start(BLUE);
        render.render_textured(builder.get());
        render.finish();
    });
}
