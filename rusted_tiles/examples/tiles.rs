extern crate glium;
extern crate rusted_tiles;

use rusted_tiles::rendering::glium_impl::GliumRenderer;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::Renderer;

fn main() {
    #[allow(unused_imports)]
    use glium::{glutin, Surface};

    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new().with_title("Example with tiles");
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut tile_renderer = TileRenderer::new([-1.0, -1.0], [0.05, 0.1]);

    tile_renderer.add_tile([0, 0], [1.0, 0.0, 0.0]);
    tile_renderer.add_polygon(
        [30, 15],
        &[[0.0, 0.2], [1.0, 0.2], [0.5, 1.0]],
        [1.0, 0.0, 0.0],
    );
    tile_renderer.add_ascii([1, 0], b'@', [1.0, 1.0, 1.0]);
    tile_renderer.add_big_ascii([5, 10], 3, b'D', [1.0, 1.0, 1.0]);
    tile_renderer.add_text([10, 15], "Hello", [0.0, 1.0, 1.0]);
    tile_renderer.add_big_text([15, 5], 5, "Big", [1.0, 1.0, 0.0]);

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

        render.start([0.0, 0.0, 0.0]);
        tile_renderer.render(&mut render);
        render.finish();
    });
}
