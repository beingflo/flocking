use nannou::prelude::*;
use nannou::draw::Draw;

use HEIGHT;
use WIDTH;

pub const AGENT_RADIUS: f32 = 5.0;
pub const AGENT_LINE_LEN: f32 = 14.0;
pub const AGENT_LINE_WIDTH: f32 = 2.0;

#[derive(Clone)]
pub struct Agent {
    id: u32,
    pos: Point2,
    vel: f32,
    dir: Vector2,
}

impl Agent {
    pub fn new(id: u32) -> Self {
        Agent { id: id, pos: Point2::new(0.0, 0.0), vel: 0.0, dir: Vector2::new(0.0, 1.0) }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_vel(&mut self, vel: f32) {
        self.vel = vel;
    }

    pub fn set_dir(&mut self, dir: Vector2) {
        self.dir = dir;
    }

    pub fn draw(&self, draw: &Draw) {
        draw.ellipse().xy(self.pos).radius(AGENT_RADIUS).color(BLACK);
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

    // Agents can only move in the direction they're oriented in
    pub fn step(&mut self, dt: f32) {
        self.pos += self.dir*dt*self.vel;

        self.wrap_pos();
    }
}
