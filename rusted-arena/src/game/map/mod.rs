pub mod builder;

use rusted_tiles::math::color::*;
use rusted_tiles::math::get_index;
use rusted_tiles::math::point::*;
use rusted_tiles::rendering::tile::TileRenderer;

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Copy, Clone, Debug)]
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

    pub fn get_neighbor(&self, pos: Point, dir: Direction) -> Option<Point> {
        match dir {
            Direction::North => self.get_with_offset(pos, 0, 1),
            Direction::East => self.get_with_offset(pos, 1, 0),
            Direction::South => self.get_with_offset(pos, 0, -1),
            Direction::West => self.get_with_offset(pos, -1, 0),
        }
    }

    pub fn get_with_offset(&self, pos: Point, delta_x: i32, delta_y: i32) -> Option<Point> {
        let x = pos.x as i32 + delta_x;
        let y = pos.y as i32 + delta_y;

        if x < 0 || x >= self.size.x as i32 || y < 0 || y >= self.size.y as i32 {
            return Option::None;
        }

        Option::Some(Point {
            x: x as u32,
            y: y as u32,
        })
    }

    pub fn can_move(&self, pos: Point) -> bool {
        self.tiles[get_index(pos.x, pos.y, self.size)] == TileType::Floor
    }
}
