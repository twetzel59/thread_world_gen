use std::sync::Arc;
use std::thread::{self, JoinHandle};
use noise::{Perlin, Seedable};
use sfml::graphics::{Image, RenderStates, RenderTarget, Sprite, Texture, Transformable};
use sfml::system::Vector2f;
use terrain::COLORS;
use tile::{Tile, TileData};

pub const TILES_ROW: usize = 10;
pub const TILE_SIZE: f32 = 64.;
pub const TILE_SIZE_U32: u32 = TILE_SIZE as u32;

pub struct Worldgen {
    perlin: Arc<Perlin>,
    handles: Vec<JoinHandle<Tile>>,
}

impl Worldgen {
    pub fn new() -> Worldgen {
        let perlin = Perlin::new();
        perlin.set_seed(1);

        Worldgen {
            perlin: Arc::new(perlin),
            handles: Vec::new(),
        }
    }

    pub fn begin(&mut self) {
        for x in 0..TILES_ROW {
            for y in 0..TILES_ROW {
                let noise_module = self.perlin.clone();

                self.handles.push(thread::spawn(move || {
                    Tile::new((x as f32, y as f32), noise_module)
                }));
            }
        }
    }

    pub fn wait_for_next(&mut self) -> Option<Tile> {
        if let Some(handle) = self.handles.pop() {
            Some(handle.join().expect("Thread join failed"))
        } else {
            None
        }
    }
}
