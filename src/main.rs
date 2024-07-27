use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::pixels::Color;

fn main() {
    // initialize SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("SearXKCD", 1440, 900)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();


    // create event loop
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // draw things
        canvas.set_draw_color(Color::WHITE);
        canvas.clear();
        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {
                    break 'running
                },
                _ => {}
            }
        }
    }
}
