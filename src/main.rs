use minifb::{Key, Window, WindowOptions, Scale};
use std::time::{Duration, Instant};

use mini_soft_render::*;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;

fn main() {
    let white_code = u32::from_str_radix("FFFFFFFF", 16).unwrap();
    let mut canvas = Canvas::with_size(WIDTH, HEIGHT);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X4,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut frames = 0;

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    let mut angle : f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        canvas.clear();

        frames += 1;
        let start = Instant::now();

        println!("{}", frames);

        canvas.draw_line(Point2D(100, 100),
                            Point2D(100 + (angle.sin() * 100.0) as i32,
                                        100 + (angle.cos() * 100.0) as i32),
                            white_code);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(canvas.get_render(), WIDTH, HEIGHT)
            .unwrap();

        let delta = start.elapsed();
        // begin_x += 1;


        angle += 0.01;

        println!("Time passed {:?}", delta);
        println!("FPS: {}", 1000.0 / delta.as_millis() as f64 );
    }
}