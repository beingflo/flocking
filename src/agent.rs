use nannou::prelude::*;
use nannou::draw::Draw;

use arena::MAX_SPEED;

#[derive(Clone)]
pub struct Agent {
    id: u32,
    pos: Point2,
    vel: Vector2,
}

impl Agent {
    pub fn new(id: u32) -> Self {
        Agent { id: id, pos: Point2::new(0.0, 0.0), vel: Vector2::new(0.0, 1.0) }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }

    pub fn set_vel(&mut self, vel: Vector2) {
        self.vel = vel;
    }

    pub fn draw(&self, draw: &Draw) {
        const RADIUS: f32 = 5.0;
        const LENGTH: f32 = 12.0;
        const WIDTH: f32 = 1.5;

        draw.ellipse().xy(self.pos).radius(RADIUS).color(BLACK);
        draw.line().start(self.pos).end(self.pos + (self.vel.normalize() * LENGTH)).thickness(WIDTH).caps_round().color(BLACK);
    }

    fn wrap_pos(&mut self, width: f32, height: f32) {
        if self.pos.x > width / 2.0 {
            self.pos.x -= width;
        }
        if self.pos.x < -width / 2.0 {
            self.pos.x += width;
        }

        if self.pos.y > height / 2.0 {
            self.pos.y -= height;
        }
        if self.pos.y < -height / 2.0 {
            self.pos.y += height;
        }
    }

    fn distance_squared(&self, other: &Agent, width: f32, height: f32) -> f32 {
        let x_dist_direct = (self.pos.x - other.pos.x).abs();
        let x_dist_indirect = width - (self.pos.x - other.pos.x).abs();

        let x_min = x_dist_direct.min(x_dist_indirect);

        let y_dist_direct = (self.pos.y - other.pos.y).abs();
        let y_dist_indirect = height - (self.pos.y - other.pos.y).abs();

        let y_min = y_dist_direct.min(y_dist_indirect);

        x_min*x_min + y_min*y_min

    }

    pub fn update(&mut self, neighbors: &[Agent], width: f32, height: f32) {
        let cohesion_coeff = 0.1;
        let alignment_coeff = 0.1;
        let separation_coeff = 0.1;

        let pull_radius = 2000.0;
        let push_radius = 1000.0;

        let mut pull = Vector2::new(0.0, 0.0);
        let mut push = Vector2::new(0.0, 0.0);
        let mut vel = Vector2::new(0.0, 0.0);

        let mut num_pull = 1;
        let mut num_push = 1;

        for n in neighbors {
            if n.id == self.id {
                continue;
            }

            let dist = self.distance_squared(n, width, height);

            if dist < pull_radius {
                pull += n.pos - self.pos;
                num_pull += 1;

                if dist < push_radius {
                    push -= n.pos - self.pos;
                    num_push += 1;
                }

                vel += n.vel;
            }
        }

        let pull = pull / (num_push as f32);
        let pull = pull * cohesion_coeff;

        let push = push * separation_coeff;

        let vel = vel / (num_pull as f32);
        let vel = vel * alignment_coeff;

        self.vel += pull;
        self.vel = self.vel.limit_magnitude(MAX_SPEED);

        self.vel += push;
        self.vel = self.vel.limit_magnitude(MAX_SPEED);

        self.vel += vel;
        self.vel = self.vel.limit_magnitude(MAX_SPEED);
    }

    // Agents can only move in the direction they're oriented in
    pub fn step(&mut self, dt: f32, width: f32, height: f32) {
        self.pos += self.vel*dt;

        self.wrap_pos(width, height);
    }
}
