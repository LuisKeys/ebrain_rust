extern crate piston_window;

use piston_window::*;
use std::thread;
use std::time::Duration;

pub fn create_window() {
    let mut window: PistonWindow =
        WindowSettings::new("eBrain Monitor", [720; 2])
            .build().unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |_c, g, _| {
            clear([0.0, 0.0, 0.0, 1.0], g);
        });
        thread::sleep(Duration::from_millis(1));
    }
}