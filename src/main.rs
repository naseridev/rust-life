use clearscreen;
use rand::Rng;
use std::thread;
use std::time::Duration;

const WIDTH: usize = 30;
const HEIGHT: usize = 30;

type World = [[bool; WIDTH]; HEIGHT];

fn draw(world: &World) {
    clearscreen::clear().expect("failed to clear screen");

    for row in world.iter() {
        for &cell in row.iter() {
            print!("{}", if cell { "#" } else { " " });
        }
        println!();
    }
}

fn evolution(world: &mut World) {
    let mut new_world = [[false; WIDTH]; HEIGHT];

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut neighbors = 0;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let ny = ((y as i32 + dy + HEIGHT as i32) % HEIGHT as i32) as usize;
                    let nx = ((x as i32 + dx + WIDTH as i32) % WIDTH as i32) as usize;

                    if world[ny][nx] {
                        neighbors += 1;
                    }
                }
            }

            new_world[y][x] = match (world[y][x], neighbors) {
                (true, 2) | (true, 3) => true,
                (false, 3) => true,
                _ => false,
            };
        }
    }

    *world = new_world;
}

fn main() {
    let mut rng = rand::rng();
    let mut world = [[false; WIDTH]; HEIGHT];

    for row in world.iter_mut() {
        for cell in row.iter_mut() {
            *cell = rng.random_ratio(1, 3);
        }
    }

    loop {
        draw(&world);
        evolution(&mut world);
        thread::sleep(Duration::from_millis(150));
    }
}
