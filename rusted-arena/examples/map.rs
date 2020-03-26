extern crate rusted_arena;
extern crate rusted_tiles;

use rusted_arena::game::component::body::*;
use rusted_arena::game::map::builder::TileMapBuilder;
use rusted_arena::game::map::*;
use rusted_tiles::math::color::*;
use rusted_tiles::math::get_index;
use rusted_tiles::math::point::*;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MapApp {
    map: TileMap,
    body: Body,
    tile_renderer: TileRenderer,
}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        self.tile_renderer.clear();
        self.map.render(&mut self.tile_renderer);

        render_body(&mut self.tile_renderer, self.map.get_size(), &self.body);

        renderer.start(BLACK);
        self.tile_renderer.render(renderer);
        renderer.finish();
    }

    fn on_button_released(&mut self, position: Point, button: MouseButton) {
        println!("Button '{:?}' released at {:?}", button, position);
        let index = get_index(position.x, position.y, self.map.get_size());
        self.body = update_position(&self.body, index);
    }

    fn on_key_released(&mut self, key: VirtualKeyCode) {
        println!("Key '{:?}' released", key);
        match key {
            VirtualKeyCode::Down => self.try_move(Direction::South),
            VirtualKeyCode::Left => self.try_move(Direction::West),
            VirtualKeyCode::Right => self.try_move(Direction::East),
            VirtualKeyCode::Up => self.try_move(Direction::North),
            VirtualKeyCode::Key1 => self.body = Body::Simple(get_position(&self.body)),
            VirtualKeyCode::Key2 => self.body = Body::Big(get_position(&self.body), 6),
            VirtualKeyCode::Key3 => self.body = Body::Snake(vec![get_position(&self.body); 20]),
            _ => (),
        }
    }
}

impl MapApp {
    fn try_move(&mut self, dir: Direction) {
        match self.get_new_position(dir, 0) {
            None => println!("Neighbor for {:?} is outside of the map!", dir),
            Some(index) => self.body = update_position(&self.body, index),
        }
    }

    fn get_new_position(&self, dir: Direction, entity: u32) -> Option<usize> {
        match self.body {
            Body::Simple(index) => self
                .map
                .get_neighbor(index, dir)
                .filter(|i| self.map.is_free(*i, entity)),
            Body::Big(index, size) => self
                .map
                .get_neighbor(index, dir)
                .filter(|i| self.map.is_square_free(*i, size, entity)),
            Body::Snake(ref indices) => self
                .map
                .get_neighbor(indices[0], dir)
                .filter(|i| self.map.is_free(*i, entity)),
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
        .set_tile(Point { x: 20, y: 15 }, TileType::Floor)
        .build();

    let mut window = GliumWindow::new("Map Example", size, tile_size);

    let app = Rc::new(RefCell::new(MapApp {
        map: tile_map,
        body: Body::Simple(410),
        tile_renderer: window.get_tile_renderer(),
    }));

    window.run(app.clone());
}
