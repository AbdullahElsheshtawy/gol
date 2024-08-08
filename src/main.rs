extern crate sdl2;
use rand::Rng;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::time::Duration;

const CELL_SIZE: usize = 10;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Abdullah's Game of Life", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let mut rng = rand::thread_rng();
    let mut grid: Vec<Vec<bool>> = (0..WINDOW_HEIGHT as usize / CELL_SIZE)
        .map(|_| {
            (0..WINDOW_WIDTH as usize / CELL_SIZE)
                .map(|_| rng.gen_bool(0.2))
                .collect()
        })
        .collect();

    'running: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => (),
            };
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for y in 0..WINDOW_HEIGHT as usize / CELL_SIZE {
            for x in 0..WINDOW_WIDTH as usize / CELL_SIZE {
                if grid[y][x] {
                    canvas
                        .fill_rect(Rect::new(
                            (x * CELL_SIZE) as i32,
                            (y * CELL_SIZE) as i32,
                            CELL_SIZE as u32,
                            CELL_SIZE as u32,
                        ))
                        .unwrap();
                }
            }
        }

        canvas.present();

        let mut new_grid = grid.clone();
        for y in 0..WINDOW_HEIGHT as usize / CELL_SIZE {
            for x in 0..WINDOW_WIDTH as usize / CELL_SIZE {
                let neighbors = count_neighbors(&grid, x, y);
                if grid[y][x] {
                    if neighbors < 2 || neighbors > 3 {
                        new_grid[y][x] = false;
                    }
                } else {
                    if neighbors == 3 {
                        new_grid[y][x] = true;
                    }
                }
            }
        }
        grid = new_grid;

        std::thread::sleep(Duration::from_millis(100));
    }
}

fn count_neighbors(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;
            if nx < grid[0].len() && ny < grid.len() && grid[ny][nx] {
                count += 1;
            }
        }
    }
    count
}
