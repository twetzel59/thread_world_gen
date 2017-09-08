extern crate noise;
extern crate sfml;

pub const VIDEO_MODE: sfml::window::VideoMode = sfml::window::VideoMode {
                        width: 640, height: 640, bits_per_pixel: 32 };

pub use worldgen::Worldgen;

pub mod terrain;
pub mod tile;
pub mod worldgen;
