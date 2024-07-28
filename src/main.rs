use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::pixels::Color;
use sdl2::ttf::{self, Font, Sdl2TtfContext};

fn main() {
    // initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("SearXKCD", 1440, 900)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture = canvas.texture_creator();

    // initialize fonts
    let ttf_context: Sdl2TtfContext = ttf::init()
        .expect("Failed to initialize TTF context");
    let font: Font = ttf_context.load_font("/usr/local/share/fonts/ttf/Fira/FiraSans-Regular.ttf", 16)
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
            canvas.copy(&text_texture.unwrap(), None, Some(Rect::new(100, 100, dimensions.0, dimensions.1))).unwrap();
        }
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                Event::TextInput { text, .. } => {
                    text_input += &text;
                },
                _ => {}
            }
        }
    }
}
