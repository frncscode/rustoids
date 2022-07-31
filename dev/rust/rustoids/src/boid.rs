use macroquad::prelude::*;
use crate::{WIDTH, HEIGHT};

fn random_vel() -> Vec2 {
    let mut vel = Vec2::new(0., 0.);
    vel.x = 1. - (rand::rand() as f64 / u32::MAX as f64) as f32;
    vel.y = 1. - vel.x;
    if rand::gen_range(1, 3) == 2 {
        vel.x *= -1.;
    }
    if rand::gen_range(1, 3) == 2 {
        vel.y *= -1.;
    }
    vel
}

pub const MAX_SPEED: f32 = 12.;
pub const MAX_FORCE: f32 = 4.;
pub const TOO_CLOSE: f32 = 20.;

pub fn random_colour() -> Color {
    Color::from_rgba(
        rand::gen_range(0, 255),
        rand::gen_range(0, 255),
        rand::gen_range(0, 255),
        255,
    )
}

#[derive(PartialEq)]
pub struct History {
    pub inner: Vec<(f32, f32)>,
    limit: usize,
}
impl History {
    pub fn new() -> Self {
        History { inner: vec![], limit: 30 }
    }

    pub fn add(&mut self, period: (f32, f32)) {
        if self.inner.len() < self.limit {
            self.inner.insert(0, period);
        } else {
            self.inner.pop();
            self.inner.insert(0, period);
        }
    }
}

#[derive(PartialEq)]
pub struct Boid {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
    pub colour: Color,
    pub history: History,
}
impl Boid {
    pub fn random() -> Self {
        Self {
            pos: Vec2::new(rand::gen_range(0., WIDTH), rand::gen_range(0., HEIGHT)),
            vel: random_vel(),
            acc: Vec2::new(0., 0.),
            colour: random_colour(),
            history: History::new(),
        }
    }

    pub fn locals<'a>(&self, flock: &'a Vec<Boid>, current_idx: usize) -> Vec<&'a Boid> {
        let mut locals = vec![];
        for i in 0..flock.len() {
            if !(i == current_idx) && self.pos.distance(flock[i].pos) < 100. {
                locals.push(&flock[i]);
            }
        }
        locals
    }

    pub fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, 3., self.colour);
    }

    pub fn draw_path(&self) {
        if self.history.inner.len() == 0 {
            return;
        }
        for i in 0..self.history.inner.len() - 1 {
            let current = self.history.inner[i];
            let next = self.history.inner[i + 1];
            draw_line(current.0, current.1, next.0, next.1, 1., WHITE);
        }
    }

    pub fn alignment(&self, locals: &Vec<&Boid>) -> Vec2 {
        if locals.len() == 0 {
            return Vec2::new(0., 0.);
        }
        let mut steering = Vec2::new(0., 0.);
        for boid in locals {
            steering += boid.vel;
        }
        steering /= locals.len() as f32;
        return (steering - self.vel).clamp_length_max(MAX_FORCE);
    }

    pub fn cohesion(&self, locals: &Vec<&Boid>) -> Vec2 {
        if locals.len() == 0 {
            return Vec2::new(0., 0.);
        }
        let mut avg = Vec2::new(0., 0.);
        for boid in locals {
            avg += boid.pos;
        }
        avg /= locals.len() as f32;

        return (avg - self.pos).clamp_length_max(MAX_FORCE);
    }

    pub fn separation(&self, locals: &Vec<&Boid>) -> Vec2 {
        let too_close: Vec<&&Boid> = locals.iter().filter(|boid| {
            boid.pos.distance(self.pos) < TOO_CLOSE
        }).collect();
        if too_close.len() == 0 {
            return Vec2::new(0., 0.);
        }

        let mut avg = Vec2::new(0., 0.);
        for boid in too_close.iter() {
            // let dist = self.pos.distance(boid.pos);
            // let mut diff = self.pos - boid.pos;
            // diff /= dist;
            // avg += diff;
            avg.x += self.pos.x - boid.pos.x;
            avg.y += self.pos.y - boid.pos.y;
        }
        avg * 0.05
    }

    pub fn update(&mut self) {
        self.vel = self.vel.clamp_length(6., MAX_SPEED);
        self.pos += self.vel;
        self.vel += self.acc;
        self.acc *= 0.;
        self.history.add((self.pos.x, self.pos.y));
    }
}
