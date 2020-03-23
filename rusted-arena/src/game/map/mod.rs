pub mod builder;

use rusted_tiles::math::color::*;
use rusted_tiles::math::get_index;
use rusted_tiles::math::point::*;
use rusted_tiles::rendering::tile::TileRenderer;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct TileMap {
    size: Point,
    tiles: Vec<TileType>,
}

impl TileMap {
    pub fn get_size(&self) -> Point {
        self.size
    }

    pub fn render(&self, renderer: &mut TileRenderer) {
        let mut pos = ZERO;

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

    pub fn can_move(&self, pos: Point) -> bool {
        self.tiles[get_index(pos.x, pos.y, self.size)] == TileType::Floor
    }
}
