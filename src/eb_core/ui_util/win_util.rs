extern crate sdl2;
use std::path::Path;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::rect::Rect;
use std::time::Duration;

pub fn create_window() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2", 1280, 720)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .build()
        .map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

    let mut event_pump = sdl_context.event_pump()?;

    let temp_surface = sdl2::surface::Surface::load_bmp(Path::new("characters.bmp"))?;
    let texture = texture_creator
        .create_texture_from_surface(&temp_surface)
        .map_err(|e| e.to_string())?;

    let frames_per_anim = 4;
    let sprite_tile_size = (32, 32);

    // Baby - walk animation
    let mut source_rect_0 = Rect::new(0, 0, sprite_tile_size.0, sprite_tile_size.0);
    let mut dest_rect_0 = Rect::new(0, 0, sprite_tile_size.0 * 4, sprite_tile_size.0 * 4);
    dest_rect_0.center_on(Point::new(-64, 120));

    let mut running = true;
    while running {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    running = false;
                }
                _ => {}
            }
        }

        source_rect_0.set_x(100);
        dest_rect_0.set_x(100);

        canvas.clear();
        canvas.copy_ex(
            &texture,
            Some(source_rect_0),
            Some(dest_rect_0),
            0.0,
            None,
            false,
            false,
        )?;
 
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}