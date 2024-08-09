pub mod index;
pub mod search;
pub mod utils;

use index::{init_index, update_index, Comic};
use search::get_results;
use utils::render_text;

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery};
use sdl2::ttf::{self, Font, Sdl2TtfContext};
use tokio;

#[tokio::main]
async fn main() {
    // initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG).unwrap();
    let window = video_subsystem
        .window("SearXKCD", 1440, 900)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let texture = canvas.texture_creator();

    // initialize xkcd index
    init_index();
    let comics: Vec<Comic> = update_index().await;

    // initialize fonts
    let ttf_context: Sdl2TtfContext = ttf::init().expect("Failed to initialize TTF context");
    let font: Font = ttf_context
        .load_font("/usr/local/share/fonts/ttf/Fira/FiraSans-Regular.ttf", 16)
        .expect("Failed to load font");
    let mut text_input: String = String::from("");

    // create scrolling variables
    let mut scroll_up = false;
    let mut scroll_down = false;
    let mut scroll_offset: i32 = 0;

    // create event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut is_search = false;
    let mut search_term: String = String::from("");
    'running: loop {
        // handle scrolling
        if scroll_up {
            scroll_offset += 10;
        } else if scroll_down {
            scroll_offset -= 10;
        }

        // draw things
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        if text_input.len() > 0 {
            render_text(
                &mut canvas,
                &texture,
                &font,
                &text_input,
                100,
                100 + scroll_offset,
            );
        }

        // draw search box
        canvas.set_draw_color(Color::BLACK);
        let search_box = Rect::new(100, 100 + scroll_offset, 1000, 30);
        canvas.draw_rect(search_box).unwrap();

        if !is_search {
            // get latest xkcd
            let img_path: String = comics[comics.len() - 1].download_img().await;
            let img_texture: Texture = texture
                .load_texture(img_path)
                .expect("Failed to load image");
            let img_info: TextureQuery = img_texture.query();
            let img_rect: Rect =
                Rect::new(200, 200 + scroll_offset, img_info.width, img_info.height);
            canvas.copy(&img_texture, None, Some(img_rect)).unwrap();
            canvas.present();
        } else {
            let results: Vec<Comic> = get_results(search_term.clone()).await;
            let mut offset_h: i32 = 150 + scroll_offset;
            for comic in results {
                let offset = (100, offset_h);
                let img_rect: Rect = comic.render(&mut canvas, &texture, &font, offset).await;
                offset_h += img_rect.height() as i32 + 70;
            }
            canvas.present();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::TextInput { text, .. } => {
                    text_input += &text;
                }
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Down => {
                        scroll_up = false;
                        scroll_down = true;
                    }
                    Keycode::Up => {
                        scroll_down = false;
                        scroll_up = true;
                    }
                    _ => {}
                },
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Backspace => {
                        text_input.pop();
                    }
                    Keycode::Return => {
                        is_search = true;
                        search_term = text_input.clone();
                    }
                    Keycode::Down => {
                        scroll_down = false;
                    }
                    Keycode::Up => {
                        scroll_up = false;
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
