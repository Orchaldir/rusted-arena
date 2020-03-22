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
    fn render(&self, renderer: &mut TileRenderer) {
        let mut pos = ZERO.clone();

        for tile in self.tiles.iter() {
            match tile {
                TileType::Floor => {
                    renderer.add_ascii(pos, b'.', WHITE);
                }
                TileType::Wall => {
                    renderer.add_ascii(pos, b'#', WHITE);
                }
            }

            pos.x += 1;

            if pos.x >= self.size.x {
                pos.x = 0;
                pos.y += 1;
            }
        }
    }

    fn can_move(&self, pos: Point) -> bool {
        self.tiles[get_index(pos.x, pos.y, self.size)] == TileType::Floor
    }
}

pub struct TileMapBuilder {
    size: Point,
    tiles: Vec<TileType>,
}

impl TileMapBuilder {
    fn new(size: Point, default: TileType) -> Self {
        let tiles = vec![default; size.get_area()];
        TileMapBuilder { size, tiles }
    }

    fn add_border(self, tile_type: TileType) -> Self {
        let size = self.size;
        self.add_rectangle(ZERO, size, tile_type)
    }

    fn add_rectangle(mut self, pos: Point, size: Point, tile_type: TileType) -> Self {
        let end = pos + size;

        for x in pos.x..end.x {
            self.tiles[get_index(x, pos.y, self.size)] = tile_type;
            self.tiles[get_index(x, end.y - 1, self.size)] = tile_type;
        }

        for y in pos.y..end.y {
            self.tiles[get_index(pos.x, y, self.size)] = tile_type;
            self.tiles[get_index(end.x - 1, y, self.size)] = tile_type;
        }

        self
    }

    fn build(self) -> TileMap {
        TileMap {
            size: self.size,
            tiles: self.tiles,
        }
    }
}

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
        let pos = Point {
            x: min(
                max(self.pos.x as i32 + delta_x, 0) as u32,
                self.map.size.x - 1,
            ),
            y: min(
                max(self.pos.y as i32 + delta_y, 0) as u32,
                self.map.size.y - 1,
            ),
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
