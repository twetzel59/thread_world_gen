use sfml::graphics::Color;

#[derive(Debug, Clone, Copy)]
pub enum Terrain {
    Undetermined,
    TallGrass,
    DesertSand,
}

pub const COLORS: [Color; Terrain::DesertSand as usize + 1] = [
    Color { r: 255, g: 255, b: 255, a:   0 },
    Color { r: 110, g: 255, b:  90, a: 255 },
    Color { r: 245, g: 255, b: 135, a: 255 },
];
