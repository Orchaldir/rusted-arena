use crate::game::component::body::Body;
use crate::game::map::TileMap;

pub fn add_entity_to_map(map: &mut TileMap, body: &Body, entity: u32) {
    match body {
        Body::Simple(index) => {
            map.add_entity(*index, entity);
        }
        Body::Big(index, size) => {
            map.add_entity_to_square(*index, *size, entity);
        }
        Body::Snake(indices) => {
            map.add_entity(indices[0], entity);
        }
    }
}

pub fn update_entity_on_map(map: &mut TileMap, body: &Body, index: usize, entity: u32) {
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
            indices
                .last()
                .map(|old_index| map.remove_entity(*old_index, entity));
            map.add_entity(index, entity);
        }
    }
}
