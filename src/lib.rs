extern crate noise;
extern crate sfml;

pub const VIDEO_MODE: sfml::window::VideoMode = sfml::window::VideoMode {
                        width: 800, height: 500, bits_per_pixel: 32 };

pub use worldgen::Worldgen;

pub mod terrain;
pub mod worldgen;
