extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use std::time::Duration;


pub fn create_window() -> Result<(), String> {

    const WINDOW_WITDH: u32 = 900;
    const WINDOW_HEIGHT: u32 = 600;
    const IMAGE_WITDH: u32 = 300;
    const IMAGE_HEIGHT: u32 = 300;
    const BUFFER_SIZE: usize = 360000;
    const PIXEL_BYTES: u32 = 3;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("SDL2", WINDOW_WITDH, WINDOW_HEIGHT)
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

    let pixel_format = PixelFormatEnum::RGB888;
    
    let mut pixels: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut index = 0;
    for index in (0..=BUFFER_SIZE-4).step_by(4) {
        pixels[index] = 0;
        pixels[index+1] = 55;
        pixels[index+2] = 0;
        //pixels[index+3] = 0;
    }
    let pitch = pixel_format.byte_size_of_pixels(IMAGE_WITDH as usize);
    let surf = sdl2::surface::Surface::from_data(&mut pixels, IMAGE_WITDH, IMAGE_HEIGHT, pitch as u32, pixel_format).unwrap();    
    let texture = texture_creator
        .create_texture_from_surface(&surf)
        .map_err(|e| e.to_string())?;

        let mut source_rect = Rect::new(0, 0, IMAGE_WITDH, IMAGE_HEIGHT);
    let mut dest_rect = Rect::new(0, 0, IMAGE_WITDH, IMAGE_HEIGHT);

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

        canvas.clear();
        canvas.copy(&texture, Some(source_rect), Some(dest_rect))?; 
        canvas.present();

        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}