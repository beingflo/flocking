extern crate kiss3d;
extern crate nalgebra as na;
extern crate rand;

use kiss3d::window::Window;
use kiss3d::scene::PlanarSceneNode;
use na::base;
use na::UnitComplex;
use na::geometry::Translation2;

const HEIGHT: u32 = 800;
const WIDTH: u32 = 800;

fn main() {
    let mut window = Window::new_with_size("Algen", WIDTH, HEIGHT);
    window.set_background_color(1.0, 1.0, 1.0);

    let mut arena = Arena::new();

    for i in 0..10 {
        arena.add_agent(&mut window);
    }

    let mut dt = 0.0;
    while window.render() {
        arena.step(dt);
        dt += 0.01
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
        let agent_radius = 10.0;
        let agent_line_len = 25.0;
        let agent_line_width = 2.0;

        let mut circle = window.add_circle(agent_radius);
        let mut line = window.add_rectangle(agent_line_width, agent_line_len);

        circle.set_color(0.0, 0.0, 0.0);
        line.set_color(0.0, 0.0, 0.0);

        let mut agent = Agent::new(circle, line);

        let x = rand::random::<f32>() * WIDTH as f32 - WIDTH as f32 / 2.0;
        let y = rand::random::<f32>() * HEIGHT as f32 - HEIGHT as f32 / 2.0;

        agent.set_vel(0.0, 0.0);
        agent.set_rot(1.0);

        agent.transform();

        self.agents.push(agent);
    }

    fn step(&mut self, dt: f32) {
        for a in &mut self.agents{
            a.set_rot(dt);
            a.transform();
        }
    }

}


struct Agent {
    pos: base::Vector2<f32>,
    vel: base::Vector2<f32>,
    rot: UnitComplex<f32>,

    circle: PlanarSceneNode,
    line: PlanarSceneNode,
}

impl Agent {
    fn new(circle: PlanarSceneNode, line: PlanarSceneNode) -> Self {
        Agent { pos: base::Vector2::new(0.0, 0.0), vel: base::Vector2::new(0.0, 0.0), rot: UnitComplex::new(0.0), circle: circle, line: line }
    }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    fn set_vel(&mut self, x: f32, y: f32) {
        self.vel.x = x;
        self.vel.y = y;
    }

    fn set_rot(&mut self, rot: f32) {
        self.rot = UnitComplex::new(rot);
    }

    // Agents can only move in the direction they're oriented in
    fn step(&mut self, dt: f32) {
        self.pos += self.vel*dt;
    }

    fn transform(&mut self) {
        self.circle.set_local_translation(Translation2::new(self.pos.x, self.pos.y));

        self.line.set_local_translation(Translation2::new(0.0, 25.0/2.0));
        self.line.set_local_rotation(UnitComplex::new(0.0));

        self.line.append_rotation(&self.rot);
        self.line.append_translation(&Translation2::new(self.pos.x + 0.0, self.pos.y));

    }
}
