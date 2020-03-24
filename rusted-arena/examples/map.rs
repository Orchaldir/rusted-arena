extern crate rusted_arena;
extern crate rusted_tiles;

use rusted_arena::game::component::body::*;
use rusted_arena::game::map::builder::TileMapBuilder;
use rusted_arena::game::map::*;
use rusted_tiles::math::color::*;
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

        render_body(&mut self.tile_renderer, &self.body);

        renderer.start(BLACK);
        self.tile_renderer.render(renderer);
        renderer.finish();
    }

    fn on_button_released(&mut self, position: Point, button: MouseButton) {
        println!("Button '{:?}' released at {:?}", button, position);
        //self.pos = position;
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
        let entered_tiles = self.get_entered_tiles(dir);

        if entered_tiles.is_empty() {
            println!("Neighbor for {:?} is outside of the map!", dir);
            return;
        }

        for entered in &entered_tiles {
            if !self.map.can_move(*entered) {
                println!("Moving {:?} is blocked by the map!", dir);
                return;
            };
        }

        self.update_pos(entered_tiles, dir);
    }

    fn get_entered_tiles(&mut self, dir: Direction) -> Vec<Point> {
        match self.body {
            Body::Simple(pos) => match self.map.get_neighbor(pos, dir) {
                None => Vec::new(),
                Some(entered) => vec![entered],
            },
            Body::Big(pos, size) => {
                let mut entered_tiles: Vec<Point> = Vec::new();

                match dir {
                    Direction::North => {
                        for i in 0..size {
                            match self.map.get_with_offset(pos, i as i32, size as i32) {
                                None => return Vec::new(),
                                Some(tile) => entered_tiles.push(tile),
                            }
                        }
                    }
                    Direction::East => {
                        for i in 0..size {
                            match self.map.get_with_offset(pos, size as i32, i as i32) {
                                None => return Vec::new(),
                                Some(tile) => entered_tiles.push(tile),
                            }
                        }
                    }
                    Direction::South => {
                        for i in 0..size {
                            match self.map.get_with_offset(pos, i as i32, -1) {
                                None => return Vec::new(),
                                Some(tile) => entered_tiles.push(tile),
                            }
                        }
                    }
                    Direction::West => {
                        for i in 0..size {
                            match self.map.get_with_offset(pos, -1, i as i32) {
                                None => return Vec::new(),
                                Some(tile) => entered_tiles.push(tile),
                            }
                        }
                    }
                }

                entered_tiles
            }
            Body::Snake(ref parts) => match self.map.get_neighbor(parts[0], dir) {
                None => Vec::new(),
                Some(entered) => vec![entered],
            },
        }
    }

    fn update_pos(&mut self, entered_tiles: Vec<Point>, dir: Direction) -> bool {
        match self.body {
            Body::Simple(_) => self.body = Body::Simple(entered_tiles[0]),
            Body::Big(pos, size) => match self.map.get_neighbor(pos, dir) {
                None => {
                    return false;
                }
                Some(neighbor) => self.body = Body::Big(neighbor, size),
            },
            Body::Snake(ref parts) => {
                let mut new_parts = vec![entered_tiles[0]];
                new_parts.extend_from_slice(parts);
                new_parts.pop();
                self.body = Body::Snake(new_parts);
            }
        }

        true
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
        body: Body::Simple(Point { x: 10, y: 10 }),
        tile_renderer: window.get_tile_renderer(),
    }));

    window.run(app.clone());
}
