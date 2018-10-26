extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

mod agent;

use agent::Agent;

use kiss3d::window::Window;

const HEIGHT: u32 = 800;
const WIDTH: u32 = 800;

const AGENT_RADIUS: f32 = 5.0;
const AGENT_LINE_LEN: f32 = 14.0;
const AGENT_LINE_WIDTH: f32 = 2.0;

const MAX_SPEED: f32 = 10.0;
const MIN_SPEED: f32 = 5.0;

fn main() {
    let mut window = Window::new_with_size("Algen", WIDTH, HEIGHT);

    window.set_background_color(1.0, 1.0, 1.0);

    let mut arena = Arena::new();

    for _ in 0..100 {
        arena.add_agent(&mut window);
    }

    while window.render() {
        arena.step(0.05);
    }
}

struct Arena {
    agents: Vec<Agent>,
}

impl Arena {
    fn new() -> Self {
        Arena { agents: Vec::new() }
    }

    fn add_agent(&mut self, window: &mut Window) {
        let mut circle = window.add_circle(AGENT_RADIUS);
        let mut line = window.add_rectangle(AGENT_LINE_WIDTH , AGENT_LINE_LEN);

        circle.set_color(0.0, 0.0, 0.0);
        line.set_color(0.0, 0.0, 0.0);

        let mut agent = Agent::new(circle, line);

        let x = rand::random::<f32>() * WIDTH as f32 - WIDTH as f32 / 2.0;
        let y = rand::random::<f32>() * HEIGHT as f32 - HEIGHT as f32 / 2.0;

        agent.set_pos(x, y);

        let v = rand::random::<f32>() * (MAX_SPEED - MIN_SPEED) + MIN_SPEED;
        agent.set_vel(v);

        let angle = rand::random::<f32>() * std::f32::consts::PI;

        agent.set_rot(angle);

        agent.transform();

        self.agents.push(agent);
    }

    fn step(&mut self, dt: f32) {
        for a in &mut self.agents{
            a.step(dt);
        }
    }

}
