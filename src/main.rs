extern crate sfml;
extern crate thread_world_gen;

use sfml::graphics::*;
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, Event, Key, style};
use thread_world_gen::*;
use terrain::COLORS;
use tile::{Tile, TileData};
use worldgen::{TILE_SIZE_U32, TILES_ROW};

fn main() {
    let mut win = create_window();

    // This is AWFUL! Note to self: If writing a graphics library, USE REFERENCE COUNTING FOR
    //  TEXTURES. WITHOUT LIFETIME ISSUES OF TEXTURE, THIS WOULDN'T BE A THING. Texture<'s>
    //  requires that I store the texture in it BY REFERENCE. This is very problematic when
    //  dynamically generating textures. Dynamic vector storage can move, invalidating the
    //  reference. No lifetime can describe how a structure could store BOTH the Sprite and
    //  a texture Sprite references. Oops! Definately, Rc would have worked much much better...

    //  Also reference counting allows texture handles to be Copy! Very important where
    //  arrays are most practical. :(
    let mut textures: [Texture; TILES_ROW * TILES_ROW] = [
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
        Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(), Texture::new(TILE_SIZE_U32, TILE_SIZE_U32).unwrap(),
    ];

    let mut n_textures: usize = 0;

    let mut tiles: Vec<Sprite> = Vec::new();
    let mut wg = Worldgen::new();
    wg.begin();

    'outer: loop {
        let mut last_pos: Option<Vector2f> = None;
        match wg.wait_for_next() {
            Some(tile) => {
                let pos = tile.world_pos();
                last_pos = Some(Vector2f::new(pos.0, pos.1));
                //textures.push(Texture::from_image(&render_tile(&tile)).unwrap());
                textures[n_textures] = Texture::from_image(&render_tile(&tile)).unwrap();
                n_textures += 1;
            },
            None => {},
        };

        if n_textures != tiles.len() {
            let mut sprite = Sprite::with_texture(&textures[n_textures - 1]);
            sprite.set_position(&last_pos.unwrap());
            tiles.push(sprite);
        }

        win.clear(&Color::rgb(85, 190, 255));
        //win.draw(&sprite);
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

fn render_tile(tile: &Tile) -> Image {
    let mut image = Image::new(TILE_SIZE_U32, TILE_SIZE_U32);
    for x in 0..TILE_SIZE_U32 {
        for y in 0..TILE_SIZE_U32 {
            image.set_pixel(x, y, &COLORS[tile.data().terrain
                                         [TileData::index_u32(x, y)] as usize]);
        }
    }

    image
}

/*


*/
