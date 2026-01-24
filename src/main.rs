extern crate rand;
use macroquad::prelude::*;
use rand::prelude::*;

struct Boid {
    position: Vec2,
    velocity: Vec2,
}

impl Boid {
    fn new(pos: Vec2, vel: Vec2) -> Self {
        Boid {
            position: pos,
            velocity: vel,
        }
    }
}

fn rotate_around(p: Vec2, pivot: Vec2, angle_rad: f32) -> Vec2 {
    let translated = p - pivot;
    let rotated = Vec2::new(
        translated.x * angle_rad.cos() - translated.y * angle_rad.sin(),
        translated.x * angle_rad.sin() + translated.y * angle_rad.cos(),
    );

    rotated + pivot
}

#[macroquad::main("Boids")]
async fn main() {
    let mut rng = rand::rng();

    let mut boids = vec![];

    for _ in 0..200 {
        let x: f32 = rng.random_range(0.0..screen_width());
        let y: f32 = rng.random_range(0.0..screen_height());
        boids.push(Boid::new(
            vec2(x, y),
            vec2(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)),
        ));
    }

    loop {
        clear_background(WHITE);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
	
	for boid in &mut boids {
            boid.position += boid.velocity;

            if boid.position.x > screen_width() { boid.position.x = 0.0; }
            if boid.position.x < 0.0 { boid.position.x = screen_width(); }
            if boid.position.y > screen_height() { boid.position.y = 0.0; }
            if boid.position.y < 0.0 { boid.position.y = screen_height(); }
	}
	
        for boid in &boids {
            let angle =
                boid.velocity.y.atan2(boid.velocity.x) + std::f32::consts::FRAC_PI_2;
            let v1 = boid.position;
            let v2 = rotate_around(Vec2::new(v1.x + 5.0, v1.y + 15.0), v1, angle);
            let v3 = rotate_around(Vec2::new(v1.x - 5.0, v1.y + 15.0), v1, angle);

            draw_triangle(v1, v2, v3, DARKGREEN);
        }

        next_frame().await;
    }
}
