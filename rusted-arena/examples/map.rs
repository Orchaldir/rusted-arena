extern crate rusted_arena;
extern crate rusted_tiles;

use rusted_arena::game::component::body::Body::*;
use rusted_arena::game::component::body::*;
use rusted_arena::game::map::builder::TileMapBuilder;
use rusted_arena::game::map::*;
use rusted_arena::game::system::movement::*;
use rusted_arena::game::system::rendering::render_bodies;
use rusted_arena::utils::ecs::storage::ComponentStorage;
use rusted_arena::utils::ecs::ECS;
use rusted_tiles::math::color::*;
use rusted_tiles::math::get_index;
use rusted_tiles::math::point::*;
use rusted_tiles::rendering::glium_impl::window::GliumWindow;
use rusted_tiles::rendering::tile::TileRenderer;
use rusted_tiles::rendering::{App, MouseButton, Renderer, VirtualKeyCode, Window};
use std::cell::RefCell;
use std::rc::Rc;

pub struct MapApp {
    ecs: ECS,
    map: TileMap,
    current_body: usize,
    tile_renderer: TileRenderer,
}

impl App for MapApp {
    fn render(&mut self, renderer: &mut dyn Renderer) {
        self.tile_renderer.clear();
        self.map.render(&mut self.tile_renderer);

        render_bodies(&mut self.ecs, &mut self.tile_renderer, self.map.get_size());

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
    pub fn new(ecs: ECS, map: TileMap, tile_renderer: TileRenderer) -> MapApp {
        MapApp {
            ecs,
            map,
            current_body: 0,
            tile_renderer,
        }
    }

    fn try_move(&mut self, dir: Direction) {
        let body_storage = self.ecs.get_storage_mgr_mut().get_mut::<Body>();

        if let Some(body) = body_storage.get_mut(self.current_body) {
            if !move_body(&mut self.map, self.current_body, body, dir) {
                println!("Neighbor for {:?} is blocked!", dir)
            }
        }
    }
}

fn main() {
    let size = Point { x: 40, y: 30 };
    let tile_size = Point { x: 20, y: 20 };
    let mut tile_map = TileMapBuilder::new(size, TileType::Floor)
        .add_border(TileType::Wall)
        .add_rectangle(
            Point { x: 20, y: 10 },
            Point { x: 10, y: 10 },
            TileType::Wall,
        )
        .set_tile(Point { x: 20, y: 15 }, TileType::Floor)
        .build();

    let mut window = GliumWindow::new("Map Example", size, tile_size);

    let mut ecs = ECS::new();

    ecs.get_storage_mgr_mut().register::<Body>();

    ecs.create_entity().with(Simple(get_index(10, 10, size)));
    ecs.create_entity().with(Big(get_index(10, 20, size), 5));
    ecs.create_entity()
        .with(Snake(vec![get_index(35, 5, size); 25]));

    add_all_to_map(&mut ecs, &mut tile_map);

    let app = Rc::new(RefCell::new(MapApp::new(
        ecs,
        tile_map,
        window.get_tile_renderer(),
    )));

    window.run(app.clone());
}
