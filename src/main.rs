extern crate rand;
use macroquad::prelude::*;
use rand::prelude::*;

const MAX_SPEED: f32 = 2.0;
const SEPARATION_WEIGHT: f32 = 1.5;
const ALIGNMENT_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 1.0;
const PERCEPTION_RADIUS: f32 = 50.0;

#[derive(Clone)]
struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
}

impl Boid {
    fn new(pos: Vec2, vel: Vec2) -> Self {
        Boid {
            position: pos,
            velocity: vel,
            acceleration: Vec2::ZERO,
        }
    }

    fn calculate_separation(&self, boids: &[Boid]) -> Vec2 {
        // Implementation here
        Vec2::new(0.0, 0.0)
    }

    fn calculate_alignment(&self, boids: &[Boid]) -> Vec2 {
        // Implementation here
        Vec2::new(0.0, 0.0)
    }

    fn calculate_cohesion(&self, boids: &[Boid]) -> Vec2 {
        // Implementation here
        Vec2::new(0.0, 0.0)
    }

    fn update(&mut self, dt: f32, snapshot: &[Boid]) {
        let sepration = self.calculate_separation(snapshot);
        let alignemt = self.calculate_alignment(snapshot);
        let cohesion = self.calculate_cohesion(snapshot);

        self.velocity += self.acceleration * dt * 60.0;
        self.velocity = self.velocity.clamp_length_max(MAX_SPEED);
        self.position += self.velocity * dt * 60.0;
        self.acceleration = Vec2::ZERO; // Reset acceleration every frame

        if self.position.x > screen_width() {
            self.position.x = 0.0;
        }
        if self.position.x < 0.0 {
            self.position.x = screen_width();
        }
        if self.position.y > screen_height() {
            self.position.y = 0.0;
        }
        if self.position.y < 0.0 {
            self.position.y = screen_height();
        }
    }

    fn draw(&self) {
        let angle = self.velocity.y.atan2(self.velocity.x) + std::f32::consts::FRAC_PI_2;
        let p = self.position;

        let v1 = p + vec2(0.0, -10.0).rotate(vec2(angle.cos(), angle.sin()));
        let v2 = p + vec2(-5.0, 5.0).rotate(vec2(angle.cos(), angle.sin()));
        let v3 = p + vec2(5.0, 5.0).rotate(vec2(angle.cos(), angle.sin()));

        draw_triangle(v1, v2, v3, DARKGREEN);
    }
}

#[macroquad::main("Boids")]
async fn main() {
    let mut rng = rand::rng();

    let mut boids = vec![];

    for _ in 0..100 {
        let x: f32 = rng.random_range(0.0..screen_width());
        let y: f32 = rng.random_range(0.0..screen_height());
        boids.push(Boid::new(
            vec2(x, y),
            vec2(rng.random_range(-1.0..1.0), rng.random_range(-1.0..1.0)).normalize(),
        ));
    }

    loop {
        clear_background(WHITE);
        let delta = get_frame_time();

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let snapshot = boids.clone();

        for boid in &mut boids {
            boid.update(delta, &snapshot);
            boid.draw();
        }

        next_frame().await;
    }
}
