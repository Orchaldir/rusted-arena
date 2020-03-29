use crate::game::component::body::{render_body, Body};
use crate::utils::ecs::storage::ComponentStorage;
use crate::utils::ecs::ECS;
use rusted_tiles::math::point::Point;
use rusted_tiles::rendering::tile::TileRenderer;

pub fn render_bodies(ecs: &mut ECS, renderer: &mut TileRenderer, size: Point) {
    let body_storage = ecs.get_storage_mgr().get::<Body>();

    for &entity in ecs.get_entities() {
        if let Some(body) = body_storage.get(entity) {
            render_body(renderer, size, body);
        }
    }
}
