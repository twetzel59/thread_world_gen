extern crate sfml;
extern crate thread_world_gen;

use sfml::graphics::*;
use sfml::window::{ContextSettings, Event, Key, style};
use thread_world_gen::*;
use worldgen::TERRAIN_SIZE;

fn main() {
    let mut win = create_window();

    let mut wg = Worldgen::new();
    wg.generate_all();
    println!("world: {:?}", wg.terrain());

    let mut img = Image::new(win.size().x, win.size().y);
    for x in 0..TERRAIN_SIZE.0 {
        for y in 0..TERRAIN_SIZE.1 {
            img.set_pixel(x as u32, y as u32,
                          &terrain::COLORS[wg.terrain()[Worldgen::index(x, y)] as usize]);
        }
    }

    let tex = Texture::from_image(&img).unwrap();

    let sprite = Sprite::with_texture(&tex);

    'outer: loop {
        win.clear(&Color::rgb(85, 190, 255));
        win.draw(&sprite);
        win.display();

        while let Some(e) = win.poll_event() {
            match e {
                Event::KeyPressed { code: Key::Escape, .. } => break 'outer,
                Event::Closed => break 'outer,
                _ => {},
            };
        }
    }
}

fn create_window() -> RenderWindow {
    RenderWindow::new(VIDEO_MODE,
                      "Multithreaded Worldgen",
                      style::DEFAULT,
                      &ContextSettings::default())
                  .unwrap()
}
