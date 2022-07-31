use macroquad::prelude::*;
use flock::Flock;

mod flock;
pub mod boid;

pub const WIDTH: f32 = 1024.;
pub const HEIGHT: f32 = 600.;

fn config() -> Conf {
    Conf {
        window_title: "rustoids.rs".to_string(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    let mut flock = Flock::spawn(200);

    loop {
        flock.update();

        clear_background(BLACK);
        flock.visible();
        flock.display();
        next_frame().await;
    }
}
