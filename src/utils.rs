use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::ttf::Font;

/// Render text with a specific font at a specific location on the canvas
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

/// Scales the given dimensions down into a Rect
pub fn fit_rect(width: u32, height: u32, rect: Rect) -> Rect {
    // get dimensions of rect
    let rect_width: u32 = rect.width();
    let rect_height: u32 = rect.height();

    // calculate scaling factors and select the biggest one
    let width_scale: f32 = rect_width as f32 / width as f32;
    let height_scale: f32 = rect_height as f32 / height as f32;
    let scale: f32 = width_scale.min(height_scale);

    // return new scaled Rect
    Rect::new(
        rect.x(),
        rect.y(),
        (rect_width as f32 * scale) as u32,
        (rect_height as f32 * scale) as u32,
    )
}
