use na::base;
use na::UnitComplex;
use na::geometry::Translation2;
use kiss3d::scene::PlanarSceneNode;

use AGENT_LINE_LEN;
use HEIGHT;
use WIDTH;

pub struct Agent {
    prop: AgentProperty,

    circle: PlanarSceneNode,
    line: PlanarSceneNode,
}

impl Agent {
    pub fn new(circle: PlanarSceneNode, line: PlanarSceneNode) -> Self {
        Agent { prop: AgentProperty::new(), circle: circle, line: line }
    }

    pub fn step(&mut self, dt: f32) {
        self.prop.step(dt);
        self.transform();
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.prop.set_pos(x,y);
    }

    pub fn set_vel(&mut self, vel: f32) {
        self.prop.set_vel(vel);
    }

    pub fn set_rot(&mut self, rot: f32) {
        self.prop.set_rot(rot);
    }

    pub fn transform(&mut self) {
        self.circle.set_local_translation(Translation2::new(self.prop.pos.x, self.prop.pos.y));

        self.line.set_local_translation(Translation2::new(0.0, AGENT_LINE_LEN / 2.0));
        self.line.set_local_rotation(UnitComplex::new(0.0));

        self.line.append_rotation(&self.prop.rot);
        self.line.append_translation(&Translation2::new(self.prop.pos.x + 0.0, self.prop.pos.y));

    }
}

struct AgentProperty {
    pos: base::Vector2<f32>,
    vel: f32,
    rot: UnitComplex<f32>,
}

impl AgentProperty {
    fn new() -> Self {
        AgentProperty { pos: base::Vector2::new(0.0, 0.0), vel: 0.0, rot: UnitComplex::new(0.0) }
    }

    fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    fn set_vel(&mut self, vel: f32) {
        self.vel = vel;
    }

    fn set_rot(&mut self, rot: f32) {
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
    fn step(&mut self, dt: f32) {
        let dir = self.rot * UnitComplex::new(std::f32::consts::PI/2.0);
        self.pos += self.vel*dt*base::Vector2::new(dir.unwrap().re, dir.unwrap().im);

        self.wrap_pos();
    }
}
