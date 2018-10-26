use kiss3d::window::Window;

use agent::Agent;
use agent::AgentRepr;

use agent::AGENT_RADIUS;
use agent::AGENT_LINE_LEN;
use agent::AGENT_LINE_WIDTH;

use HEIGHT;
use WIDTH;

const MAX_SPEED: f32 = 10.0;
const MIN_SPEED: f32 = 5.0;

pub struct Arena {
    agents: Vec<Agent>,
    agent_reprs: Vec<AgentRepr>,
}

impl Arena {
    pub fn new() -> Self {
        Arena { agents: Vec::new(), agent_reprs: Vec::new() }
    }

    pub fn add_agent(&mut self, window: &mut Window) {
        let mut circle = window.add_circle(AGENT_RADIUS);
        let mut line = window.add_rectangle(AGENT_LINE_WIDTH , AGENT_LINE_LEN);

        circle.set_color(0.0, 0.0, 0.0);
        line.set_color(0.0, 0.0, 0.0);

        let mut agent_repr = AgentRepr::new(circle, line);
        let mut agent = Agent::new();

        let x = rand::random::<f32>() * WIDTH as f32 - WIDTH as f32 / 2.0;
        let y = rand::random::<f32>() * HEIGHT as f32 - HEIGHT as f32 / 2.0;

        agent.set_pos(x, y);

        let v = rand::random::<f32>() * (MAX_SPEED - MIN_SPEED) + MIN_SPEED;
        agent.set_vel(v);

        let angle = rand::random::<f32>() * std::f32::consts::PI;

        agent.set_rot(angle);

        agent_repr.transform(&agent);

        self.agents.push(agent);
        self.agent_reprs.push(agent_repr);
    }

    pub fn step(&mut self, dt: f32) {
        let agents2 = self.agents.clone();

        for a in &mut self.agents {
            a.update(&agents2);
        }

        for i in 0..self.agents.len() {
            self.agents[i].step(dt);
            self.agent_reprs[i].transform(&self.agents[i]);
        }
    }

}
