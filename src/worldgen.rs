use noise::{NoiseModule, Perlin, Seedable};
use terrain::Terrain;
use VIDEO_MODE;

pub const TERRAIN_SIZE: (usize, usize) = (VIDEO_MODE.width as usize, VIDEO_MODE.height as usize);
const TERRAIN_LEN: usize = TERRAIN_SIZE.0 * TERRAIN_SIZE.1;
const DIVISOR: f32 = 343.;

pub struct Worldgen {
    perlin: Perlin,
    terrain: [Terrain; TERRAIN_LEN],
}

impl Worldgen {
    pub fn new() -> Worldgen {
        let perlin = Perlin::new();
        perlin.set_seed(1);

        Worldgen {
            perlin,
            terrain: [Terrain::Undetermined; TERRAIN_LEN],
        }
    }

    pub fn terrain(&self) -> &[Terrain] {
        &self.terrain
    }

    pub fn generate_all(&mut self) {
        for x in 0..TERRAIN_SIZE.0 {
            for y in 0..TERRAIN_SIZE.1 {
                let pixel = &mut self.terrain[Self::index(x, y)];
                let (x, y) = (x as f32 / DIVISOR, y as f32 / DIVISOR);

                *pixel = if self.perlin.get([x, y]) > 0.2 {
                    Terrain::DesertSand
                } else {
                    Terrain::TallGrass
                };
            }
        }
    }

    pub fn index(x: usize, y: usize) -> usize {
        x * TERRAIN_SIZE.1 + y
    }
}
