use na::base;
use na::UnitComplex;
use na::geometry::Translation2;
use kiss3d::window::Window;
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

    pub fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.circle.set_color(r, g, b);
        self.line.set_color(r, g, b);
    }

    pub fn transform(&mut self, data: &Agent) {
        self.circle.set_local_translation(Translation2::new(data.pos.x, data.pos.y));

        self.line.set_local_translation(Translation2::new(0.0, AGENT_LINE_LEN / 2.0));
        self.line.set_local_rotation(UnitComplex::new(0.0));

        self.line.append_rotation(&data.rot);
        self.line.append_translation(&Translation2::new(data.pos.x + 0.0, data.pos.y));

    }
}

#[derive(Clone)]
pub struct Agent {
    id: u32,
    pos: base::Vector2<f32>,
    vel: f32,
    rot: UnitComplex<f32>,
}

impl Agent {
    pub fn new(id: u32) -> Self {
        Agent { id: id, pos: base::Vector2::new(0.0, 0.0), vel: 0.0, rot: UnitComplex::new(0.0) }
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

    fn distance_squared(&self, other: &Agent) -> f32 {
        let x_dist_direct = (self.pos.x - other.pos.x).abs();
        let x_dist_indirect = WIDTH as f32 - (self.pos.x - other.pos.x).abs();

        let x_min = x_dist_direct.min(x_dist_indirect);

        let y_dist_direct = (self.pos.y - other.pos.y).abs();
        let y_dist_indirect = HEIGHT as f32 - (self.pos.y - other.pos.y).abs();

        let y_min = y_dist_direct.min(y_dist_indirect);

        x_min*x_min + y_min*y_min

    }

    pub fn update(&mut self, neighbors: &[Agent], window: &mut Window) -> Option<Vec<usize>> {
        if self.id != 0 {
            return None;
        }

        let mut close = Vec::new();

        for (i, a) in neighbors.iter().enumerate() {
            if self.distance_squared(a) < 30000.0 {
                close.push(i);
            }
        }

        Some(close)
    }

    // Agents can only move in the direction they're oriented in
    pub fn step(&mut self, dt: f32) {
        let dir = self.rot * UnitComplex::new(std::f32::consts::PI/2.0);
        self.pos += self.vel*dt*base::Vector2::new(dir.unwrap().re, dir.unwrap().im);

        self.wrap_pos();
    }
}
