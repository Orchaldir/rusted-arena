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
    entities: HashMap<usize, u32>,
}

impl TileMap {
    pub fn get_size(&self) -> Point {
        self.size
    }

    fn assert_inside(&self, index: usize) {
        assert!(
            index < self.tiles.len(),
            "Index {} is outside the map!",
            index
        );
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
        self.assert_inside(index);

        if !self.tiles[index].is_walkable() {
            return false;
        }

        match self.entities.get(&index) {
            None => true,
            Some(e) if *e == entity => true,
            Some(_) => false,
        }
    }

    pub fn is_square_free(&self, index: usize, size: u32, entity: u32) -> bool {
        self.assert_inside(index);
        execute_function_on_square(self.size, index, size, |i: usize| self.is_free(i, entity))
    }

    // occupying entities

    pub fn get_entity(&mut self, index: usize) -> Option<&u32> {
        self.assert_inside(index);
        self.entities.get(&index)
    }

    pub fn remove_entity(&mut self, index: usize, entity: u32) -> bool {
        self.assert_inside(index);

        match self.entities.remove(&index) {
            None => panic!("Could not remove entity {} at {}!", entity, index),
            Some(other) if other != entity => panic!(
                "Removed entity {} instead of {} at {}!",
                other, entity, index
            ),
            _ => {}
        }

        true
    }

    pub fn remove_entity_from_square(&mut self, index: usize, size: u32, entity: u32) -> bool {
        self.assert_inside(index);
        execute_function_on_square(self.size, index, size, |i: usize| {
            self.remove_entity(i, entity)
        })
    }

    pub fn add_entity(&mut self, index: usize, entity: u32) -> bool {
        self.assert_inside(index);

        match self.entities.insert(index, entity) {
            Some(other) if other == entity => panic!("Entity {} is already at {}!", entity, index),
            Some(other) => panic!(
                "Adding entity {} blocked by {} at {}!",
                entity, other, index
            ),
            _ => {}
        }

        true
    }

    pub fn add_entity_to_square(&mut self, index: usize, size: u32, entity: u32) -> bool {
        self.assert_inside(index);
        execute_function_on_square(self.size, index, size, |i: usize| {
            self.add_entity(i, entity)
        })
    }
}

fn execute_function_on_square<F>(map_size: Point, index: usize, size: u32, mut func: F) -> bool
where
    F: FnMut(usize) -> bool,
{
    let pos = get_point(index, map_size);

    if pos.x > map_size.x - size || pos.y > map_size.y - size {
        return false;
    }

    for dx in 0..size {
        let x = pos.x + dx;

        for dy in 0..size {
            let y = pos.y + dy;

            if !func(get_index(x, y, map_size)) {
                return false;
            }
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::map::builder::TileMapBuilder;
    use crate::game::map::TileType::*;

    const OUTSIDE: usize = 12;
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
            entities: vec![(0usize, 0u32)].into_iter().collect(),
        };

        assert_eq!(map.is_free(0, 0), true);
        assert_eq!(map.is_free(0, 1), false);
        assert_eq!(map.is_free(0, 2), false);
    }

    #[test]
    #[should_panic(expected = "Index 12 is outside the map!")]
    fn test_is_free_outside() {
        let map = TileMapBuilder::new(SIZE, Floor).build();

        map.is_free(OUTSIDE, 0);
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
            entities: vec![(5usize, 0u32)].into_iter().collect(),
        };

        assert_is_square_free(&map, 0, FREE_RESULTS);
        assert_is_square_free(&map, 1, BLOCKED_RESULTS);
    }

    #[test]
    #[should_panic(expected = "Index 12 is outside the map!")]
    fn test_is_square_free_outside() {
        let map = TileMapBuilder::new(SIZE, Floor).build();

        map.is_square_free(OUTSIDE, 2, 0);
    }

    fn assert_is_square_free(map: &TileMap, entity: u32, results: [bool; 12]) -> () {
        for (i, result) in results.iter().enumerate() {
            assert_eq!(map.is_square_free(i, 2, entity), *result);
        }
    }

    #[test]
    #[should_panic(expected = "Index 12 is outside the map!")]
    fn test_get_entity_outside() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.get_entity(OUTSIDE);
    }

    #[test]
    fn test_add_entity() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.add_entity(5, 42);

        assert_eq!(map.get_entity(0), None);
        assert_eq!(map.get_entity(5), Some(&42));
    }

    #[test]
    #[should_panic(expected = "Index 12 is outside the map!")]
    fn test_add_entity_outside() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.add_entity(OUTSIDE, 42);
    }

    #[test]
    #[should_panic(expected = "Entity 42 is already at 2!")]
    fn test_add_entity_twice() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.add_entity(2, 42);
        map.add_entity(2, 42);
    }

    #[test]
    #[should_panic(expected = "Adding entity 2 blocked by 1 at 5!")]
    fn test_add_entity_different() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.add_entity(5, 1);
        map.add_entity(5, 2);
    }

    #[test]
    fn test_remove_entity() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.add_entity(5, 10);
        map.remove_entity(5, 10);
        map.add_entity(5, 20);

        assert_eq!(map.get_entity(5), Some(&20));
    }

    #[test]
    #[should_panic(expected = "Could not remove entity 10 at 5!")]
    fn test_remove_entity_not_there() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.remove_entity(5, 10);
    }

    #[test]
    #[should_panic(expected = "Removed entity 10 instead of 20 at 5!")]
    fn test_remove_wrong_entity() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.add_entity(5, 10);
        map.remove_entity(5, 20);
    }

    #[test]
    fn test_add_entity_to_square() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.add_entity_to_square(5, 2, 9);

        assert_eq!(map.get_entity(0), None);
        assert_eq!(map.get_entity(1), None);
        assert_eq!(map.get_entity(2), None);
        assert_eq!(map.get_entity(3), None);
        assert_eq!(map.get_entity(4), None);
        assert_eq!(map.get_entity(5), Some(&9));
        assert_eq!(map.get_entity(6), Some(&9));
        assert_eq!(map.get_entity(7), None);
        assert_eq!(map.get_entity(8), None);
        assert_eq!(map.get_entity(9), Some(&9));
        assert_eq!(map.get_entity(10), Some(&9));
        assert_eq!(map.get_entity(11), None);
    }

    #[test]
    fn test_remove_entity_from_square() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();

        map.add_entity_to_square(5, 2, 9);
        map.remove_entity_from_square(5, 2, 9);

        for i in 0..12 {
            assert_eq!(map.get_entity(i), None);
        }
    }
}
