extern crate rusted_tiles;

use rusted_tiles::math::color::*;
use rusted_tiles::math::get_index;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Floor,
    Wall,
}

pub struct TileMap {
    size: [u32; 2],
    tiles: Vec<TileType>,
}

impl TileMap {
    fn new(size: [u32; 2], default: TileType) -> TileMap {
        let mut tiles = vec![default; (size[0] * size[1]) as usize];

        for x in 0..size[0] {
            tiles[get_index([x, 0], size)] = TileType::Wall;
            tiles[get_index([x, size[1] - 1], size)] = TileType::Wall;
        }

        for y in 0..size[1] {
            tiles[get_index([0, y], size)] = TileType::Wall;
            tiles[get_index([size[0] - 1, y], size)] = TileType::Wall;
        }

        TileMap { size, tiles }
    }

    fn render(&mut self, renderer: &mut TileRenderer) {
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
            if x >= self.size[0] {
                x = 0;
                y += 1;
            }
        }
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
            VirtualKeyCode::Down => self.pos[1] -= 1,
            VirtualKeyCode::Left => self.pos[0] -= 1,
            VirtualKeyCode::Right => self.pos[0] += 1,
            VirtualKeyCode::Up => self.pos[1] += 1,
            _ => (),
        }
    }
}

fn main() {
    let size = [40, 30];
    let mut window = GliumWindow::new("Map Example", size, [20, 20]);
    let app = Rc::new(RefCell::new(MapApp {
        map: TileMap::new(size, TileType::Floor),
        pos: [10, 10],
        tile_renderer: window.get_tile_renderer(),
    }));

    window.run(app.clone());
}
