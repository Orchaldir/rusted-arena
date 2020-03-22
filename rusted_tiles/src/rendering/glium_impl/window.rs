use super::GliumRenderer;
use crate::math::point::*;
use crate::rendering::tile::TileRenderer;
use crate::rendering::{App, Window};
use glium::glutin;
use std::cell::RefCell;
use std::cmp;
use std::rc::Rc;

pub struct GliumWindow {
    title: &'static str,
    tiles: Point,
    tile_size: Point,
    size: Point,
}

impl GliumWindow {
    pub fn new(title: &'static str, tiles: Point, tile_size: Point) -> GliumWindow {
        let size = tiles * tile_size;
        GliumWindow {
            title,
            tiles,
            tile_size,
            size,
        }
    }

    pub fn default_size(title: &'static str) -> GliumWindow {
        GliumWindow::new(title, Point { x: 80, y: 60 }, Point { x: 10, y: 10 })
    }
}

impl Window for GliumWindow {
    fn get_tile_renderer(&self) -> TileRenderer {
        TileRenderer::new(ZERO, self.tile_size)
    }

    fn run(&mut self, app: Rc<RefCell<dyn App>>) -> ! {
        let size = glutin::dpi::LogicalSize::new(self.size.x, self.size.y);
        let event_loop = glutin::event_loop::EventLoop::new();
        let wb = glutin::window::WindowBuilder::new()
            .with_title(self.title)
            .with_resizable(false)
            .with_inner_size(size);
        let cb = glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();

        let mut renderer = GliumRenderer::new(display, self.size);
        let mut mouse_index = ZERO;
        let height = self.tiles.y;
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
                        mouse_index.x = position.x as u32 / tile_size.x;
                        mouse_index.y = cmp::max(height - position.y as u32 / tile_size.y, 1) - 1;
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
