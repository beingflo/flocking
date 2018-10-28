use nannou::prelude::*;
use nannou::draw::Draw;

use agent::Agent;

use HEIGHT;
use WIDTH;

const MAX_SPEED: f32 = 10.0;
const MIN_SPEED: f32 = 5.0;

pub struct Arena {
    agents: Vec<Agent>,

    id_counter: u32,
}

impl Arena {
    pub fn new() -> Self {
        Arena { agents: Vec::new(), id_counter: 0 }
    }

    pub fn add_agent(&mut self) {
        let mut agent = Agent::new(self.id_counter);
        self.id_counter += 1;

        let x = random_f32() * WIDTH as f32 - WIDTH as f32 / 2.0;
        let y = random_f32() * HEIGHT as f32 - HEIGHT as f32 / 2.0;

        agent.set_pos(x, y);

        let v = random_f32() * (MAX_SPEED - MIN_SPEED) + MIN_SPEED;
        agent.set_vel(v);

        let direction = Vector2::new(random_f32()-0.5, random_f32()-0.5).normalize();

        agent.set_dir(direction);

        self.agents.push(agent);
    }

    pub fn draw(&self, draw: &Draw) {
        for a in &self.agents {
            a.draw(draw);
        }
    }

    pub fn step(&mut self, dt: f32) {
        let agents2 = self.agents.clone();

        for a in &mut self.agents {
            a.step(dt);
        }
    }

}
