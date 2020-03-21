use super::GliumRenderer;
use crate::rendering::tile::TileRenderer;
use crate::rendering::{App, Window};
use glium::glutin;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub struct GliumWindow {
    title: &'static str,
    tiles: [u32; 2],
    tile_size: [u32; 2],
    size: [u32; 2],
}

impl GliumWindow {
    pub fn new(title: &'static str, tiles: [u32; 2], tile_size: [u32; 2]) -> GliumWindow {
        let size = [tiles[0] * tile_size[0], tiles[1] * tile_size[1]];
        GliumWindow {
            title,
            tiles,
            tile_size,
            size,
        }
    }
}

impl Window for GliumWindow {
    fn get_tile_renderer(&self) -> TileRenderer {
        let tile_size = [self.tile_size[0] as f32, self.tile_size[1] as f32];
        TileRenderer::new([0.0, 0.0], tile_size)
    }

    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> ! {
        let size = glutin::dpi::LogicalSize::new(self.size[0], self.size[1]);
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title(self.title)
            .with_resizable(false)
            .with_inner_size(size);
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let mut renderer = GliumRenderer::new(display, self.size);
        let mut mouse_index = [0 as u32, 0 as u32];
        let height = self.tiles[1];
        let tile_size = self.tile_size;

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
                    glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                        if input.state == glutin::event::ElementState::Released {
                            if let Some(key) = input.virtual_keycode {
                                let mut reference = app.borrow_mut();
                                reference.on_key_released(key);
                            }
                        }
                    }
                    glutin::event::WindowEvent::CursorMoved { position, .. } => {
                        mouse_index[0] = position.x as u32 / tile_size[0];
                        mouse_index[1] = cmp::max(height - position.y as u32 / tile_size[1], 1) - 1;
                        return;
                    }
                    glutin::event::WindowEvent::MouseInput { state, button, .. } => {
                        if state == glutin::event::ElementState::Released {
                            let mut reference = app.borrow_mut();
                            reference.on_button_released(mouse_index, button);
                        }
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
