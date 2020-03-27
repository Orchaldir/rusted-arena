use crate::game::component::body::{update_position, Body};
use crate::game::map::{Direction, TileMap};

pub fn add_entity_to_map(map: &mut TileMap, body: &Body, entity: usize) {
    match body {
        Body::Simple(index) => {
            map.add_entity(*index, entity);
        }
        Body::Big(index, size) => {
            map.add_entity_to_square(*index, *size, entity);
        }
        Body::Snake(indices) => {
            for index in indices {
                map.add_entity(*index, entity);
            }
        }
    }
}

fn update_entity_on_map(map: &mut TileMap, body: &Body, index: usize, entity: usize) {
    match body {
        Body::Simple(old_index) => {
            map.remove_entity(*old_index, entity);
            map.add_entity(index, entity);
        }
        Body::Big(old_index, size) => {
            map.remove_entity_from_square(*old_index, *size, entity);
            map.add_entity_to_square(index, *size, entity);
        }
        Body::Snake(indices) => {
            let length = indices.len();
            let last_index = indices[length - 1];
            let count = indices.iter().filter(|&i| *i == last_index).count();

            if count == 1 {
                map.remove_entity(last_index, entity);
            }

            map.add_entity(index, entity);
        }
    }
}

pub fn move_body(map: &mut TileMap, entity: usize, body: &mut Body, dir: Direction) -> bool {
    match get_new_position(map, entity, body, dir) {
        None => false,
        Some(index) => {
            update_entity_on_map(map, body, index, entity);
            update_position(body, index);
            true
        }
    }
}

fn get_new_position(map: &TileMap, entity: usize, body: &Body, dir: Direction) -> Option<usize> {
    match body {
        Body::Simple(index) => map
            .get_neighbor(*index, dir)
            .filter(|i| map.is_free(*i, entity)),
        Body::Big(index, size) => map
            .get_neighbor(*index, dir)
            .filter(|i| map.is_square_free(*i, *size, entity)),
        Body::Snake(ref indices) => map
            .get_neighbor(indices[0], dir)
            .filter(|i| map.is_free(*i, entity)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::component::body::Body::*;
    use crate::game::map::builder::TileMapBuilder;
    use crate::game::map::Direction::*;
    use crate::game::map::TileType::Floor;
    use rusted_tiles::math::point::Point;

    const SIZE: Point = Point { x: 3, y: 3 };
    const ENTITY: usize = 42;

    // add_entity_to_map

    #[test]
    fn test_add_entity_to_map_simple() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        add_entity_to_map(&mut map, &Simple(0), ENTITY);

        assert_simple(&mut map)
    }

    #[test]
    fn test_add_entity_to_map_big() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        add_entity_to_map(&mut map, &Big(4, 2), ENTITY);

        assert_big(&mut map);
    }

    #[test]
    fn test_add_entity_to_map_sanke() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        add_entity_to_map(&mut map, &Snake(vec![1, 4, 7]), ENTITY);

        assert_snake(&mut map);
    }

    // move_body

    #[test]
    fn test_move_body_simple() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        let mut body = Simple(1);

        add_entity_to_map(&mut map, &body, ENTITY);

        assert_eq!(move_body(&mut map, ENTITY, &mut body, West), true);
        assert_eq!(body, Simple(0));
        assert_simple(&mut map);
    }

    #[test]
    fn test_move_body_simple_blocked() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        let mut body = Simple(0);

        add_entity_to_map(&mut map, &body, ENTITY);

        assert_eq!(move_body(&mut map, ENTITY, &mut body, West), false);
        assert_eq!(body, Simple(0));
        assert_simple(&mut map);
    }

    #[test]
    fn test_move_body_big() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        let mut body = Big(1, 2);

        add_entity_to_map(&mut map, &body, ENTITY);

        assert_eq!(move_body(&mut map, ENTITY, &mut body, North), true);
        assert_eq!(body, Big(4, 2));
        assert_big(&mut map);
    }

    #[test]
    fn test_move_body_big_blocked() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        let mut body = Big(4, 2);

        add_entity_to_map(&mut map, &body, ENTITY);

        assert_eq!(move_body(&mut map, ENTITY, &mut body, North), false);
        assert_eq!(body, Big(4, 2));
        assert_big(&mut map);
    }

    #[test]
    fn test_move_body_snake() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        let mut body = Snake(vec![4, 7, 8]);

        add_entity_to_map(&mut map, &body, ENTITY);

        assert_eq!(move_body(&mut map, ENTITY, &mut body, South), true);
        assert_eq!(body, Snake(vec![1, 4, 7]));
        assert_snake(&mut map);
    }

    #[test]
    fn test_move_body_snake_blocked() {
        let mut map = TileMapBuilder::new(SIZE, Floor).build();
        let mut body = Snake(vec![1, 4, 7]);

        add_entity_to_map(&mut map, &body, ENTITY);

        assert_eq!(move_body(&mut map, ENTITY, &mut body, South), false);
        assert_eq!(body, Snake(vec![1, 4, 7]));
        assert_snake(&mut map);
    }

    // asserts

    fn assert_simple(map: &mut TileMap) -> () {
        assert_eq!(map.get_entity(0), Some(&ENTITY));
        for i in 1..9 {
            assert_eq!(map.get_entity(i), None);
        }
    }

    fn assert_big(map: &mut TileMap) {
        assert_eq!(map.get_entity(0), None);
        assert_eq!(map.get_entity(1), None);
        assert_eq!(map.get_entity(2), None);
        assert_eq!(map.get_entity(3), None);
        assert_eq!(map.get_entity(4), Some(&ENTITY));
        assert_eq!(map.get_entity(5), Some(&ENTITY));
        assert_eq!(map.get_entity(6), None);
        assert_eq!(map.get_entity(7), Some(&ENTITY));
        assert_eq!(map.get_entity(8), Some(&ENTITY));
    }

    fn assert_snake(map: &mut TileMap) {
        assert_eq!(map.get_entity(0), None);
        assert_eq!(map.get_entity(1), Some(&ENTITY));
        assert_eq!(map.get_entity(2), None);
        assert_eq!(map.get_entity(3), None);
        assert_eq!(map.get_entity(4), Some(&ENTITY));
        assert_eq!(map.get_entity(5), None);
        assert_eq!(map.get_entity(6), None);
        assert_eq!(map.get_entity(7), Some(&ENTITY));
        assert_eq!(map.get_entity(8), None);
    }
}
