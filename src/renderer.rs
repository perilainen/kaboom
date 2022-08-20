use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};
use specs::{Join, ReadStorage};

use crate::components::{Health, Position, Sprite};

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
    ReadStorage<'a, Health>,
);

pub fn render_health(canvas: &mut WindowCanvas, data: SystemData) -> Result<(), String> {
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let mut font = ttf_context.load_font("assets/lalissa.ttf", 128)?;
    let healt: String = data.2.join().next().unwrap().health.to_string();
    let surface = font
        .render(&healt)
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();
    let text = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGBA(195, 217, 255, 255));
    let target = Rect::new(20, 20, 40, 40);
    canvas.copy(&text, None, Some(target))?;
    // canvas.present();
    Ok(())
}

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();
    let (width, heigt) = canvas.output_size()?;

    for (pos, sprite) in (&data.0, &data.1).join() {
        let current_frame = sprite.region;

        let screen_postion = pos.0 + Point::new(width as i32 / 2, heigt as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_postion,
            current_frame.width(),
            current_frame.height(),
        );
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;
    }
    render_health(canvas, data)?;

    canvas.present();
    Ok(())
}
