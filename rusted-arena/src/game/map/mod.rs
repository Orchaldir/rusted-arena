pub mod builder;

use rusted_tiles::math::color::*;
use rusted_tiles::math::point::*;
use rusted_tiles::math::{get_index, get_point};
use rusted_tiles::rendering::tile::TileRenderer;
use std::collections::HashMap;

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

impl TileType {
    pub fn is_walkable(self) -> bool {
        match self {
            Self::Floor => true,
            Self::Wall => false,
        }
    }
}

pub struct TileMap {
    size: Point,
    tiles: Vec<TileType>,
    occupying_entities: HashMap<usize, u32>,
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

    pub fn get_neighbor(&self, index: usize, dir: Direction) -> Option<usize> {
        match dir {
            Direction::North => self.get_with_offset(index, 0, 1),
            Direction::East => self.get_with_offset(index, 1, 0),
            Direction::South => self.get_with_offset(index, 0, -1),
            Direction::West => self.get_with_offset(index, -1, 0),
        }
    }

    pub fn get_with_offset(&self, index: usize, delta_x: i32, delta_y: i32) -> Option<usize> {
        let pos = get_point(index, self.size);
        let x = pos.x as i32 + delta_x;
        let y = pos.y as i32 + delta_y;

        if x < 0 || x >= self.size.x as i32 || y < 0 || y >= self.size.y as i32 {
            return Option::None;
        }

        Option::Some(get_index(x as u32, y as u32, self.size))
    }

    pub fn is_free(&self, index: usize, entity: u32) -> bool {
        assert!(
            index < self.tiles.len(),
            "Index {} is outside the map!",
            index
        );

        if !self.tiles[index].is_walkable() {
            return false;
        }

        match self.occupying_entities.get(&index) {
            None => true,
            Some(e) if *e == entity => true,
            Some(_) => false,
        }
    }

    pub fn is_square_free(&self, index: usize, size: u32, entity: u32) -> bool {
        assert!(
            index < self.tiles.len(),
            "Index {} is outside the map!",
            index
        );
        let pos = get_point(index, self.size);

        if pos.x > self.size.x - size || pos.y > self.size.y - size {
            return false;
        }

        for dx in 0..size {
            let x = pos.x + dx;

            for dy in 0..size {
                let y = pos.y + dy;

                if !self.is_free(get_index(x, y, self.size), entity) {
                    return false;
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::map::builder::TileMapBuilder;
    use crate::game::map::TileType::*;

    const SIZE: Point = Point { x: 4, y: 3 };

    const FREE_RESULTS: [bool; 12] = [
        true, true, true, false, true, true, true, false, false, false, false, false,
    ];
    const BLOCKED_RESULTS: [bool; 12] = [
        false, false, true, false, false, false, true, false, false, false, false, false,
    ];

    #[test]
    fn test_is_walkable() {
        assert_eq!(TileType::Floor.is_walkable(), true);
        assert_eq!(TileType::Wall.is_walkable(), false);
    }

    #[test]
    fn test_is_free() {
        let map = TileMapBuilder::new(SIZE, Floor)
            .set_tile(xy(0, 0), Wall)
            .build();

        for i in 0..12 {
            assert_eq!(map.is_free(i, 0), i != 0);
        }
    }

    #[test]
    fn test_is_free_with_occupied_map() {
        let map = TileMap {
            size: xy(1, 1),
            tiles: vec![Floor],
            occupying_entities: vec![(0usize, 0u32)].into_iter().collect(),
        };

        assert_eq!(map.is_free(0, 0), true);
        assert_eq!(map.is_free(0, 1), false);
        assert_eq!(map.is_free(0, 2), false);
    }

    #[test]
    #[should_panic(expected = "Index 12 is outside the map!")]
    fn test_is_free_outside() {
        let map = TileMapBuilder::new(SIZE, Floor).build();

        map.is_free(12, 0);
    }

    #[test]
    fn test_is_square_free() {
        let map = TileMapBuilder::new(SIZE, Floor).build();

        assert_is_square_free(&map, 0, FREE_RESULTS);
    }

    #[test]
    fn test_is_square_free_with_wall() {
        let map = TileMapBuilder::new(SIZE, Floor)
            .set_tile(xy(1, 1), Wall)
            .build();

        assert_is_square_free(&map, 0, BLOCKED_RESULTS);
    }

    #[test]
    fn test_is_square_free_with_occupied_map() {
        let map = TileMap {
            size: xy(4, 3),
            tiles: vec![Floor; 12],
            occupying_entities: vec![(5usize, 0u32)].into_iter().collect(),
        };

        assert_is_square_free(&map, 0, FREE_RESULTS);
        assert_is_square_free(&map, 1, BLOCKED_RESULTS);
    }

    #[test]
    #[should_panic(expected = "Index 12 is outside the map!")]
    fn test_is_square_free_outside() {
        let map = TileMapBuilder::new(SIZE, Floor).build();

        map.is_square_free(12, 2, 0);
    }

    fn assert_is_square_free(map: &TileMap, entity: u32, results: [bool; 12]) -> () {
        for (i, result) in results.iter().enumerate() {
            assert_eq!(map.is_square_free(i, 2, entity), *result);
        }
    }
}
