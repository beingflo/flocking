use nannou::prelude::*;
use nannou::draw::Draw;

use agent::Agent;

pub struct Arena {
    agents: Vec<Agent>,

    width: f32,
    height: f32,

    id_counter: u32,
}

impl Arena {
    pub fn new(width: f32, height: f32) -> Self {
        Arena { agents: Vec::new(), width: width, height: height, id_counter: 0 }
    }

    pub fn update_size(&mut self, size: Vector2) {
        self.width = size.x;
        self.height = size.y;
    }

    pub fn add_agent(&mut self) {
        const MAX_SPEED: f32 = 50.0;
        const MIN_SPEED: f32 = 20.0;

        let mut agent = Agent::new(self.id_counter);
        self.id_counter += 1;

        let x = (random_f32() * self.width) - (self.width / 2.0);
        let y = (random_f32() * self.height) - (self.height / 2.0);

        agent.set_pos(x, y);

        let v = (random_f32() * (MAX_SPEED - MIN_SPEED)) + MIN_SPEED;
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

    pub fn update(&mut self, dt: f32) {
        let agents_copy = self.agents.clone();

        for a in &mut self.agents {
            a.update(&agents_copy, self.width, self.height);
        }
    }


    pub fn step(&mut self, dt: f32) {
        for a in &mut self.agents {
            a.step(dt, self.width, self.height);
        }
    }

}
