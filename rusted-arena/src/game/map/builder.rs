use super::*;
use rusted_tiles::math::get_index;
use rusted_tiles::math::point::*;

pub struct TileMapBuilder {
    size: Point,
    tiles: Vec<TileType>,
}

impl TileMapBuilder {
    pub fn new(size: Point, default: TileType) -> Self {
        let tiles = vec![default; size.get_area()];
        TileMapBuilder { size, tiles }
    }

    pub fn add_border(self, tile_type: TileType) -> Self {
        let size = self.size;
        self.add_rectangle(ZERO, size, tile_type)
    }

    pub fn add_rectangle(mut self, pos: Point, size: Point, tile_type: TileType) -> Self {
        let end = pos + size;

        for x in pos.x..end.x {
            self.tiles[get_index(x, pos.y, self.size)] = tile_type;
            self.tiles[get_index(x, end.y - 1, self.size)] = tile_type;
        }

        for y in pos.y..end.y {
            self.tiles[get_index(pos.x, y, self.size)] = tile_type;
            self.tiles[get_index(end.x - 1, y, self.size)] = tile_type;
        }

        self
    }

    pub fn set_tile(mut self, pos: Point, tile_type: TileType) -> Self {
        self.tiles[get_index(pos.x, pos.y, self.size)] = tile_type;
        self
    }

    pub fn build(self) -> TileMap {
        TileMap {
            size: self.size,
            tiles: self.tiles,
            occupying_entities: HashMap::new(),
        }
    }
}
