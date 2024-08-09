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
    let mut grid = new_grid();

    'running: loop {
        for event in sdl_context.event_pump().unwrap().poll_iter() {
            if let Event::Quit { .. } = event {
                break 'running;
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for (y, _) in grid
            .iter()
            .enumerate()
            .take(WINDOW_HEIGHT as usize / CELL_SIZE)
        {
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
        update_grid(&mut grid);

        std::thread::sleep(Duration::from_millis(100));
    }
}

fn new_grid() -> Vec<Vec<bool>> {
    let mut rng = rand::thread_rng();
    (0..WINDOW_HEIGHT as usize / CELL_SIZE)
        .map(|_| {
            (0..WINDOW_WIDTH as usize / CELL_SIZE)
                .map(|_| rng.gen_bool(0.2))
                .collect()
        })
        .collect()
}
fn update_grid(grid: &mut Vec<Vec<bool>>) {
    let mut new_grid = grid.clone();
    for y in 0..WINDOW_HEIGHT as usize / CELL_SIZE {
        for x in 0..WINDOW_WIDTH as usize / CELL_SIZE {
            let neighbors = count_neighbors(grid, x, y);
            if grid[y][x] {
                if !(2..=3).contains(&neighbors) {
                    new_grid[y][x] = false;
                }
            } else if neighbors == 3 {
                new_grid[y][x] = true;
            }
        }
    }
    *grid = new_grid;
}

fn count_neighbors(grid: &[Vec<bool>], x: usize, y: usize) -> usize {
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
