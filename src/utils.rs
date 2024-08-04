use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::ttf::Font;

// render text with a specific font at a specific location on the canvas
pub fn render_text(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    texture_creator: &sdl2::render::TextureCreator<sdl2::video::WindowContext>,
    font: &Font,
    text: &str,
    x: i32,
    y: i32,
) {
    let text_surface: Surface = font
        .render(text)
        .blended(Color::BLACK)
        .expect("Failed to render text");
    let text_texture = text_surface.as_texture(texture_creator);
    let dimensions = font.size_of(text).unwrap();
    canvas
        .copy(
            &text_texture.unwrap(),
            None,
            Some(Rect::new(x, y, dimensions.0, dimensions.1)),
        )
        .unwrap();
}
