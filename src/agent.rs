use na::base;
use na::UnitComplex;
use na::geometry::Translation2;
use kiss3d::scene::PlanarSceneNode;

use HEIGHT;
use WIDTH;

pub const AGENT_RADIUS: f32 = 5.0;
pub const AGENT_LINE_LEN: f32 = 14.0;
pub const AGENT_LINE_WIDTH: f32 = 2.0;

pub struct AgentRepr {
    circle: PlanarSceneNode,
    line: PlanarSceneNode,
}

impl AgentRepr {
    pub fn new(circle: PlanarSceneNode, line: PlanarSceneNode) -> Self {
        AgentRepr { circle: circle, line: line }
    }

    pub fn transform(&mut self, data: &Agent) {
        self.circle.set_local_translation(Translation2::new(data.pos.x, data.pos.y));

        self.line.set_local_translation(Translation2::new(0.0, AGENT_LINE_LEN / 2.0));
        self.line.set_local_rotation(UnitComplex::new(0.0));

        self.line.append_rotation(&data.rot);
        self.line.append_translation(&Translation2::new(data.pos.x + 0.0, data.pos.y));

    }
}

pub struct Agent {
    pos: base::Vector2<f32>,
    vel: f32,
    rot: UnitComplex<f32>,
}

impl Agent {
    pub fn new() -> Self {
        Agent { pos: base::Vector2::new(0.0, 0.0), vel: 0.0, rot: UnitComplex::new(0.0) }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_vel(&mut self, vel: f32) {
        self.vel = vel;
    }

    pub fn set_rot(&mut self, rot: f32) {
        self.rot = UnitComplex::new(rot);
    }

    fn wrap_pos(&mut self) {
        if self.pos.x > WIDTH as f32 / 2.0 {
            self.pos.x = -(WIDTH as f32) / 2.0;
        }
        if self.pos.x < -(WIDTH as f32) / 2.0 {
            self.pos.x = WIDTH as f32 / 2.0;
        }

        if self.pos.y > HEIGHT as f32 / 2.0 {
            self.pos.y = -(HEIGHT as f32) / 2.0;
        }
        if self.pos.y < -(HEIGHT as f32) / 2.0 {
            self.pos.y = HEIGHT as f32 / 2.0;
        }
    }

    // Agents can only move in the direction they're oriented in
    pub fn step(&mut self, dt: f32) {
        let dir = self.rot * UnitComplex::new(std::f32::consts::PI/2.0);
        self.pos += self.vel*dt*base::Vector2::new(dir.unwrap().re, dir.unwrap().im);

        self.wrap_pos();
    }
}
