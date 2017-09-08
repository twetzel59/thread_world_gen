use std::sync::Arc;
use noise::{NoiseModule, Perlin};
use terrain::Terrain;
use worldgen::TILE_SIZE;

const DIVISOR: f32 = 343.;
const TERRAIN_LEN: usize = TERRAIN_SIZE * TERRAIN_SIZE;
const TERRAIN_SIZE: usize = TILE_SIZE as usize;

pub type WorldPos = (f32, f32);

pub struct Tile {
    data: TileData,
    world_pos: WorldPos,
}

impl Tile {
    pub fn new(world_pos: WorldPos, perlin: Arc<Perlin>) -> Tile {
        let mut t = Tile {
            data: TileData {
                terrain: [Terrain::Undetermined; TERRAIN_LEN],
            },
            world_pos,
        };

        t.gen_terrain(&perlin);

        t
    }

    pub fn data(&self) -> &TileData {
        &self.data
    }

    pub fn world_pos(&self) -> WorldPos {
        self.world_pos
    }

    fn gen_terrain(&mut self, perlin: &Perlin) {
        for x in 0..TERRAIN_SIZE {
            for y in 0..TERRAIN_SIZE {
                let pixel = &mut self.data.terrain[TileData::index(x, y)];
                let (x, y) = (x as f32 / DIVISOR + self.world_pos.0 * TILE_SIZE,
                              y as f32 / DIVISOR + self.world_pos.1 * TILE_SIZE);

                *pixel = if perlin.get([x, y]) > 0.2 {
                    Terrain::DesertSand
                } else {
                    Terrain::TallGrass
                };
            }
        }
    }
}

pub struct TileData {
    pub terrain: [Terrain; TERRAIN_LEN],
}

impl TileData {
    pub fn index(x: usize, y: usize) -> usize {
        x * TERRAIN_SIZE + y
    }

    pub fn index_u32(x: u32, y: u32) -> usize {
        Self::index(x as usize, y as usize)
    }
}
