use crate::game::component::body::Body;
use crate::game::component::graphic::{Graphic, GraphicData};
use crate::utils::ecs::storage::ComponentStorage;
use crate::utils::ecs::ECS;
use rusted_tiles::math::get_point;
use rusted_tiles::math::point::Point;
use rusted_tiles::rendering::tile::TileRenderer;
use std::cmp::min;

pub fn render_entities(ecs: &mut ECS, renderer: &mut TileRenderer, size: Point) {
    let entities = ecs.get_entities_of_2::<Body, Graphic>();
    let body_storage = ecs.get_storage_mgr().get::<Body>();
    let graphic_storage = ecs.get_storage_mgr().get::<Graphic>();

    for entity in entities {
        if let Some(body) = body_storage.get(entity) {
            if let Some(graphic) = graphic_storage.get(entity) {
                render_entity(renderer, size, body, graphic);
            }
        }
    }
}

fn render_entity(renderer: &mut TileRenderer, size: Point, body: &Body, graphic: &Graphic) {
    match body {
        Body::Simple(index) => render_graphic(renderer, size, *index, graphic.get(0)),
        Body::Big(index, s) => render_big_graphic(renderer, size, *index, *s, graphic.get(0)),
        Body::Snake(indices) => {
            for i in (0..indices.len()).rev() {
                let data_index = min(i, 1);
                render_graphic(renderer, size, indices[i], graphic.get(data_index))
            }
        }
    }
}

fn render_graphic(renderer: &mut TileRenderer, size: Point, index: usize, data: &GraphicData) {
    match data {
        GraphicData::Ascii(ascii, color) => {
            renderer.add_ascii(get_point(index, size), *ascii, *color)
        }
    }
}

fn render_big_graphic(
    renderer: &mut TileRenderer,
    size: Point,
    index: usize,
    tile_size: u32,
    data: &GraphicData,
) {
    match data {
        GraphicData::Ascii(ascii, color) => {
            renderer.add_big_ascii(get_point(index, size), tile_size, *ascii, *color)
        }
    }
}
