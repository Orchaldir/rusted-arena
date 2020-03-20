extern crate glium;
extern crate rusted_tiles;

use rusted_tiles::rendering::ascii::AsciiBuilder;
use rusted_tiles::rendering::glium_impl::GliumRenderer;
use rusted_tiles::rendering::Renderer;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Example with ascii");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut builder = AsciiBuilder::default();

    builder.add_u8([-0.5, -0.5], [0.5, 0.5], b'a', [1.0, 0.0, 0.0]);
    builder.add_char([0.0, -0.5], [0.5, 0.5], 'b', [0.0, 1.0, 0.0]);
    builder.add_string([0.0, 0.2], [0.1, 0.1], "Test?", [1.0, 1.0, 1.0]);
    builder.add_string(
        [-1.0, 0.4],
        [0.05, 0.1],
        "Non-Ascii Symbols are replaced with '🎉'!",
        [1.0, 1.0, 0.0],
    );

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

        render.start([0.0, 0.0, 1.0]);
        render.render_textured(builder.get());
        render.finish();
    });
}
