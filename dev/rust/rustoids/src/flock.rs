use macroquad::prelude::*;
use crate::boid::Boid;
use crate::{WIDTH, HEIGHT};

pub struct Flock {
    pub flock: Vec<Boid>,
    pub align_coef: f32,
    pub cohese_coef: f32,
    pub sep_coef: f32,
    _population: usize,
}

impl Flock {
    pub fn spawn(population: usize) -> Self {
        let mut flock = vec![];
        for _ in 0..population {
            flock.push(Boid::random());
        }

        Self {
            flock,
            align_coef: 0.7,
            cohese_coef: 0.4,
            sep_coef: 0.8,
            _population: population,
        }
    }

    pub fn update(&mut self) {
        for i in 0..self.flock.len() {
            let locals = self.flock[i].locals(&self.flock, i);
            let alignment = self.flock[i].alignment(&locals) * self.align_coef;
            let cohesion = self.flock[i].cohesion(&locals) * self.cohese_coef;
            let separation = self.flock[i].separation(&locals) * self.sep_coef;
            
            self.flock[i].acc += alignment * self.align_coef;
            self.flock[i].acc += cohesion * self.cohese_coef;
            self.flock[i].acc += separation * self.sep_coef;
            self.flock[i].update();
        }
    }

    pub fn display(&self) {
        for boid in self.flock.iter() {
            boid.draw();
            boid.draw_path();
        }
    }

    pub fn visible(&mut self) {
        for boid in self.flock.iter_mut() {
            if boid.pos.x < 0. {
                boid.pos.x = WIDTH;
                boid.history.inner.clear();
            } else if boid.pos.x > WIDTH {
                boid.pos.x = 0.;
                boid.history.inner.clear();
            } 
            if boid.pos.y < 0. {
                boid.pos.y = HEIGHT;
                boid.history.inner.clear();
            } else if boid.pos.y > HEIGHT {
                boid.pos.y = 0.;
                boid.history.inner.clear();
            }
        }
    }
}
