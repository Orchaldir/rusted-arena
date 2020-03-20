use super::GliumRenderer;
use crate::rendering::tile::TileRenderer;
use crate::rendering::{App, Window};
use glium::glutin;
use std::cell::RefCell;
use std::rc::Rc;

pub struct GliumWindow {
    title: &'static str,
    tiles: [u32; 2],
    tile_size: [u32; 2],
}

impl GliumWindow {
    pub fn new(title: &'static str, tiles: [u32; 2], tile_size: [u32; 2]) -> GliumWindow {
        GliumWindow {
            title,
            tiles,
            tile_size,
        }
    }
}

impl Window for GliumWindow {
    fn get_tile_renderer(&self) -> TileRenderer {
        let tile_size = [2.0 / self.tiles[0] as f32, 2.0 / self.tiles[1] as f32];
        TileRenderer::new([-1.0, -1.0], tile_size)
    }

    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> ! {
        let size = glutin::dpi::LogicalSize::new(
            self.tiles[0] * self.tile_size[0],
            self.tiles[1] * self.tile_size[1],
        );
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title(self.title)
            .with_inner_size(size);
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        let mut renderer = GliumRenderer::new(display);

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

            let mut reference = app.borrow_mut();
            reference.render(&mut renderer);
        });
    }
}
