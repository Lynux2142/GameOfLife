extern crate minifb;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::time::Duration;

const SQUARE_SIZE: i32 = 10;
const WIDTH: i32 = 80 * SQUARE_SIZE;
const HEIGHT: i32 = 60 * SQUARE_SIZE;
const UP: i32 = -WIDTH * SQUARE_SIZE;
const DOWN: i32 = WIDTH * SQUARE_SIZE;
const LEFT: i32 = -SQUARE_SIZE;
const RIGHT: i32 = SQUARE_SIZE;

fn key_events(window: &Window, direction: &mut i32) {
    if window.is_key_pressed(Key::Up, KeyRepeat::No) { *direction = UP }
    if window.is_key_pressed(Key::Down, KeyRepeat::No) { *direction = DOWN }
    if window.is_key_pressed(Key::Left, KeyRepeat::No) { *direction = LEFT }
    if window.is_key_pressed(Key::Right, KeyRepeat::No) { *direction = RIGHT }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; (WIDTH * HEIGHT) as usize];
    let mut window = match Window::new("test", WIDTH as usize, HEIGHT as usize, WindowOptions::default()) {
        Ok(mut window) => {
            window.limit_update_rate(Some(Duration::from_millis(75)));
            window
        },
        Err(e) => { println!("Unable to create window {}", e); return; }
    };
    let mut head: i32 = HEIGHT / 2 * WIDTH + WIDTH / 2;
    let mut direction: i32 = -WIDTH * SQUARE_SIZE;

    loop {
        if window.is_key_down(Key::Escape) { break; }
        key_events(&window, &mut direction);
        for y in 1..SQUARE_SIZE {
            for x in 1..SQUARE_SIZE {
                buffer[(head + (y * WIDTH + x)) as usize] = 0xFFFFFF;
            }
        }
        if (head > WIDTH && direction == UP) ||
            (head < (WIDTH * (HEIGHT - SQUARE_SIZE - 1)) && direction == DOWN) ||
            ((head % WIDTH) > 0 && direction == LEFT) ||
            ((head % WIDTH) < (WIDTH - SQUARE_SIZE - 1) && direction == RIGHT) {
            head += direction;
        }
        window.update_with_buffer(&buffer, WIDTH as usize, HEIGHT as usize).unwrap();
    }
}
