extern crate rusted_tiles;

use rusted_tiles::math::color::*;
use rusted_tiles::math::get_index;
use rusted_tiles::math::point::*;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::cmp::{max, min};
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Floor,
    Wall,
}

pub struct TileMap {
    size: Point,
    tiles: Vec<TileType>,
}

impl TileMap {
    fn new(size: Point, default: TileType) -> TileMap {
        let mut tiles = vec![default; size.get_area()];

        for x in 0..size.x {
            tiles[get_index(x, 0, size)] = TileType::Wall;
            tiles[get_index(x, size.y - 1, size)] = TileType::Wall;
        }

        for y in 0..size.y {
            tiles[get_index(0, y, size)] = TileType::Wall;
            tiles[get_index(size.x - 1, y, size)] = TileType::Wall;
        }

        TileMap { size, tiles }
    }

    fn render(&self, renderer: &mut TileRenderer) {
        let mut y = 0;
        let mut x = 0;

        for tile in self.tiles.iter() {
            match tile {
                TileType::Floor => {
                    renderer.add_ascii([x, y], b'.', WHITE);
                }
                TileType::Wall => {
                    renderer.add_ascii([x, y], b'#', WHITE);
                }
            }

            x += 1;
            if x >= self.size.x {
                x = 0;
                y += 1;
            }
        }
    }

    fn can_move(&self, pos: [u32; 2]) -> bool {
        self.tiles[get_index(pos[0], pos[1], self.size)] == TileType::Floor
    }
}

pub struct MapApp {
    map: TileMap,
    pos: [u32; 2],
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

    fn on_button_released(&mut self, position: [u32; 2], button: MouseButton) {
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
        let pos = [
            min(
                max(self.pos[0] as i32 + delta_x, 0) as u32,
                self.map.size.x - 1,
            ),
            min(
                max(self.pos[1] as i32 + delta_y, 0) as u32,
                self.map.size.y - 1,
            ),
        ];

        if self.map.can_move(pos) {
            self.pos = pos;
        }
    }
}

fn main() {
    let size = [40, 30];
    let mut window = GliumWindow::new("Map Example", size, [20, 20]);
    let app = Rc::new(RefCell::new(MapApp {
        map: TileMap::new(
            Point {
                x: size[0],
                y: size[1],
            },
            TileType::Floor,
        ),
        pos: [10, 10],
        tile_renderer: window.get_tile_renderer(),
    }));

    window.run(app.clone());
}
