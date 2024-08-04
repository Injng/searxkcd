pub mod index;

use index::{init_index, update_index, Comic};

use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Texture, TextureQuery};
use sdl2::surface::Surface;
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
    let mut comics: Vec<Comic> = update_index().await;

    // initialize fonts
    let ttf_context: Sdl2TtfContext = ttf::init().expect("Failed to initialize TTF context");
    let font: Font = ttf_context
        .load_font("/usr/local/share/fonts/ttf/Fira/FiraSans-Regular.ttf", 16)
        .expect("Failed to load font");
    let mut text_input: String = String::from("");

    // create event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // draw things
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        if text_input.len() > 0 {
            let text_surface: Surface = font
                .render(&text_input)
                .blended(Color::BLACK)
                .expect("Failed to render text");
            let text_texture = text_surface.as_texture(&texture);
            let dimensions = font.size_of(&text_input).unwrap();
            canvas
                .copy(
                    &text_texture.unwrap(),
                    None,
                    Some(Rect::new(100, 100, dimensions.0, dimensions.1)),
                )
                .unwrap();
        }

        // get latest xkcd
        let img_path: String = comics[comics.len() - 1].download_img().await;
        let img_texture: Texture = texture
            .load_texture(img_path)
            .expect("Failed to load image");
        let img_info: TextureQuery = img_texture.query();
        let img_rect: Rect = Rect::new(200, 200, img_info.width, img_info.height);
        canvas.copy(&img_texture, None, Some(img_rect)).unwrap();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::TextInput { text, .. } => {
                    text_input += &text;
                }
                _ => {}
            }
        }
    }
}
