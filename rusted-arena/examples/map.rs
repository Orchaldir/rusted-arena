extern crate rusted_arena;
extern crate rusted_tiles;

use rusted_arena::game::component::body::*;
use rusted_arena::game::map::builder::TileMapBuilder;
use rusted_arena::game::map::*;
use rusted_arena::game::system::movement::*;
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
    bodies: Vec<Body>,
    current_body: usize,
    tile_renderer: TileRenderer,
}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        self.tile_renderer.clear();
        self.map.render(&mut self.tile_renderer);

        for body in &self.bodies {
            render_body(&mut self.tile_renderer, self.map.get_size(), body);
        }

        renderer.start(BLACK);
        self.tile_renderer.render(renderer);
        renderer.finish();
    }

    fn on_button_released(&mut self, position: Point, button: MouseButton) {
        println!("Button '{:?}' released at {:?}", button, position);
    }

    fn on_key_released(&mut self, key: VirtualKeyCode) {
        println!("Key '{:?}' released", key);
        match key {
            VirtualKeyCode::Down => self.try_move(Direction::South),
            VirtualKeyCode::Left => self.try_move(Direction::West),
            VirtualKeyCode::Right => self.try_move(Direction::East),
            VirtualKeyCode::Up => self.try_move(Direction::North),
            VirtualKeyCode::Key1 => self.current_body = 0,
            VirtualKeyCode::Key2 => self.current_body = 1,
            VirtualKeyCode::Key3 => self.current_body = 2,
            _ => (),
        }
    }
}

impl MapApp {
    pub fn new(map: TileMap, tile_renderer: TileRenderer) -> MapApp {
        MapApp {
            map,
            bodies: Vec::new(),
            current_body: 0,
            tile_renderer,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        let entity = self.bodies.len();
        add_entity_to_map(&mut self.map, &body, entity);
        self.bodies.push(body);
    }

    fn try_move(&mut self, dir: Direction) {
        let body = &self.bodies[self.current_body];

        match self.get_new_position(body, dir, self.current_body) {
            None => println!("Neighbor for {:?} is outside of the map!", dir),
            Some(index) => {
                update_entity_on_map(&mut self.map, body, index, self.current_body);
                self.bodies[self.current_body] = update_position(body, index);
            }
        }
    }

    fn get_new_position(&self, body: &Body, dir: Direction, entity: usize) -> Option<usize> {
        match body {
            Body::Simple(index) => self
                .map
                .get_neighbor(*index, dir)
                .filter(|i| self.map.is_free(*i, entity)),
            Body::Big(index, size) => self
                .map
                .get_neighbor(*index, dir)
                .filter(|i| self.map.is_square_free(*i, *size, entity)),
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

    let app = Rc::new(RefCell::new(MapApp::new(
        tile_map,
        window.get_tile_renderer(),
    )));

    app.borrow_mut()
        .add_body(Body::Simple(get_index(10, 10, size)));
    app.borrow_mut()
        .add_body(Body::Big(get_index(10, 20, size), 5));
    app.borrow_mut()
        .add_body(Body::Snake(vec![get_index(35, 5, size); 25]));

    window.run(app.clone());
}
