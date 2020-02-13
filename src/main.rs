extern crate minifb;

use minifb::{Key, KeyRepeat, MouseMode, Window, WindowOptions};
use std::time::Duration;

const SQUARE_SIZE: i32 = 10;
const WIDTH: i32 = 108;
const HEIGHT: i32 = 108;
const WIN_WIDTH: i32 = WIDTH * SQUARE_SIZE;
const WIN_HEIGHT: i32 = HEIGHT * SQUARE_SIZE;

fn display(buffer: &mut Vec<u32>, grid: &Vec<Vec<i32>>)
{
    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            buffer[y * WIDTH as usize + x] = if grid[y][x] == 1 { 0xFFFFFF } else { 0 };
        }
    }
}

fn make_grid(window: &mut Window, buffer: &mut Vec<u32>, grid: &mut Vec<Vec<i32>>) {
    window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
        let x: usize = (mouse.0 / SQUARE_SIZE as f32) as usize;
        let y: usize = (mouse.1 / SQUARE_SIZE as f32) as usize;

        if window.is_key_pressed(Key::Space, KeyRepeat::No) {
            grid[y][x] = 1 ^ grid[y][x];
        }
        if window.is_key_pressed(Key::C, KeyRepeat::No) {
            for y in 0..HEIGHT as usize {
                for x in 0..WIDTH as usize {
                    if grid[y][x] == 1 {
                        println!("grid[{}][{}] = 1;", y, x);
                    }
                }
            }
        }
    });
    display(buffer, grid);
}

fn get_neighbors(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> i32 {
    let mut neighbors: i32 = 0;

    for j in -1..=1 {
        for i in -1..=1 {
            if i != 0 || j != 0 {
                let tmpx = x + i;
                let tmpy = y + j;

                if tmpx >= 0 && tmpx < WIDTH && tmpy >= 0 && tmpy < HEIGHT {
                    if grid[tmpy as usize][tmpx as usize] == 1 { neighbors += 1; }
                }
            }
        }
    }
    neighbors
}

fn expand_life(buffer: &mut Vec<u32>, grid: &mut Vec<Vec<i32>>) {
    let mut next_grid = grid.clone();

    for y in 0..HEIGHT as usize {
        for x in 0..WIDTH as usize {
            let neighbors = get_neighbors(grid, x as i32, y as i32);

            if grid[y][x] == 0 && neighbors == 3 {
                next_grid[y][x] = 1;
            }
            if grid[y][x] == 1 && (neighbors < 2 || neighbors > 3) {
                next_grid[y][x] = 0;
            }
        }
    }
    *grid = next_grid;
    display(buffer, grid);
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; (WIN_WIDTH * WIN_HEIGHT) as usize];
    let mut window = match Window::new("test", WIN_WIDTH as usize, WIN_HEIGHT as usize, WindowOptions::default()) {
        Ok(mut window) => {
            window.limit_update_rate(Some(Duration::from_millis(0)));
            window
        },
        Err(e) => { println!("Unable to create window {}", e); return; }
    };
    let mut grid: Vec<Vec<i32>> = vec![vec![0; WIDTH as usize]; HEIGHT as usize];
    let mut is_make_mode: bool = true;
    grid[0][24] = 1;
    grid[1][22] = 1;
    grid[1][24] = 1;
    grid[2][12] = 1;
    grid[2][13] = 1;
    grid[2][20] = 1;
    grid[2][21] = 1;
    grid[2][34] = 1;
    grid[2][35] = 1;
    grid[3][11] = 1;
    grid[3][15] = 1;
    grid[3][20] = 1;
    grid[3][21] = 1;
    grid[3][34] = 1;
    grid[3][35] = 1;
    grid[4][0] = 1;
    grid[4][1] = 1;
    grid[4][10] = 1;
    grid[4][16] = 1;
    grid[4][20] = 1;
    grid[4][21] = 1;
    grid[5][0] = 1;
    grid[5][1] = 1;
    grid[5][10] = 1;
    grid[5][14] = 1;
    grid[5][16] = 1;
    grid[5][17] = 1;
    grid[5][22] = 1;
    grid[5][24] = 1;
    grid[6][10] = 1;
    grid[6][16] = 1;
    grid[6][24] = 1;
    grid[7][11] = 1;
    grid[7][15] = 1;
    grid[8][12] = 1;
    grid[8][13] = 1;

    loop {
        if window.is_key_down(Key::Escape) { break; }
        if window.is_key_pressed(Key::Enter, KeyRepeat::No) {
            is_make_mode = !is_make_mode;
            if is_make_mode {
                window.limit_update_rate(Some(Duration::from_millis(0)));
            } else {
                window.limit_update_rate(Some(Duration::from_millis(0)));
            }
        }
        if is_make_mode {
            make_grid(&mut window, &mut buffer, &mut grid);
        } else {
            expand_life(&mut buffer, &mut grid);
        }
        window.update_with_buffer(&mut buffer, WIDTH as usize, HEIGHT as usize).unwrap();
    }
}
