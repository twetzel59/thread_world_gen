extern crate sfml;

use sfml::graphics::*;
use sfml::window::{ContextSettings, Event, Key, VideoMode, style};

const VIDEO_MODE: VideoMode = VideoMode { width: 800, height: 500, bits_per_pixel: 32 };

fn main() {
    let mut win = create_window();

    'outer: loop {
        win.clear(&Color::rgb(85, 190, 255));
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
