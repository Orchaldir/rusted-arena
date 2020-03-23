extern crate rusted_arena;
extern crate rusted_tiles;

use rusted_arena::game::map::builder::TileMapBuilder;
use rusted_arena::game::map::*;
use rusted_tiles::math::color::*;
use rusted_tiles::math::point::*;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;

pub struct MapApp {
    map: TileMap,
    pos: Point,
    tile_renderer: TileRenderer,
}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        self.tile_renderer.clear();
        self.map.render(&mut self.tile_renderer);
        self.tile_renderer.add_ascii(self.pos, b'@', RED);

        renderer.start(BLACK);
        self.tile_renderer.render(renderer);
        renderer.finish();
    }

    fn on_button_released(&mut self, position: Point, button: MouseButton) {
        println!("Button '{:?}' released at {:?}", button, position);
        self.pos = position;
    }

    fn on_key_released(&mut self, key: VirtualKeyCode) {
        println!("Key '{:?}' released", key);
        match key {
            VirtualKeyCode::Down => self.try_move(0, -1),
            VirtualKeyCode::Left => self.try_move(-1, 0),
            VirtualKeyCode::Right => self.try_move(1, 0),
            VirtualKeyCode::Up => self.try_move(0, 1),
            _ => (),
        }
    }
}

impl MapApp {
    fn try_move(&mut self, delta_x: i32, delta_y: i32) {
        let size = self.map.get_size();
        let pos = Point {
            x: min(max(self.pos.x as i32 + delta_x, 0) as u32, size.x - 1),
            y: min(max(self.pos.y as i32 + delta_y, 0) as u32, size.y - 1),
        };

        if self.map.can_move(pos) {
            self.pos = pos;
        }
    }
}

fn main() {
    let size = Point { x: 40, y: 30 };
    let tile_size = Point { x: 20, y: 20 };
    let tile_map = TileMapBuilder::new(size, TileType::Floor)
        .add_border(TileType::Wall)
        .add_rectangle(
            Point { x: 20, y: 10 },
            Point { x: 10, y: 10 },
            TileType::Wall,
        )
        .build();

    let mut window = GliumWindow::new("Map Example", size, tile_size);

    let app = Rc::new(RefCell::new(MapApp {
        map: tile_map,
        pos: Point { x: 10, y: 10 },
        tile_renderer: window.get_tile_renderer(),
    }));

    window.run(app.clone());
}
