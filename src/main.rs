extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

mod agent;
mod arena;

use arena::Arena;
use kiss3d::window::Window;

const HEIGHT: u32 = 800;
const WIDTH: u32 = 800;

fn main() {
    let mut window = Window::new_with_size("Algen", WIDTH, HEIGHT);

    window.set_background_color(1.0, 1.0, 1.0);

    let mut arena = Arena::new();

    for _ in 0..1000 {
        arena.add_agent(&mut window);
    }

    while window.render() {
        arena.step(0.1, &mut window);
    }
}

