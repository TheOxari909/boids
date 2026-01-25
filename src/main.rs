extern crate rand;
use macroquad::prelude::*;
use rand::prelude::*;

const MAX_SPEED: f32 = 300.0;
const MIN_SPEED: f32 = 150.0;
const MAX_FORCE: f32 = 40.0;

const BOIDS: usize = 500;

const SEPARATION_WEIGHT: f32 = 1.7;
const ALIGNMENT_WEIGHT: f32 = 0.6;
const COHESION_WEIGHT: f32 = 1.2;

const SEPARATION_RADIUS: f32 = 25.0;
const PERCEPTION_RADIUS: f32 = 50.0;
const PERCEPTION_RADIUS_SQ: f32 = PERCEPTION_RADIUS * PERCEPTION_RADIUS;
const SEPARATION_RADIUS_SQ: f32 = SEPARATION_RADIUS * SEPARATION_RADIUS;

const SIM_SPEED: f32 = 1.0;

#[derive(Clone)]
struct Boid {
    position: Vec2,
    velocity: Vec2,
    neighbors: usize,
}

impl Boid {
    fn new(pos: Vec2, vel: Vec2) -> Self {
        Boid {
            position: pos,
            velocity: vel,
            neighbors: 0,
        }
    }

    fn do_compute(
        &mut self,
        other: &Boid,
        pos_avg: &mut Vec2,
        vel_avg: &mut Vec2,
        close_d: &mut Vec2,
    ) -> Vec2 {
        let diff = self.position - other.position;
        let dist_sq = diff.length_squared();
        if dist_sq < 0.01 {
            return Vec2::ZERO;
        }

        if dist_sq >= PERCEPTION_RADIUS_SQ {
            return Vec2::ZERO;
        }
        if dist_sq < SEPARATION_RADIUS_SQ {
            *close_d += self.position - other.position;
        } else if dist_sq < PERCEPTION_RADIUS_SQ {
            *pos_avg += other.position;
            *vel_avg += other.velocity;
            self.neighbors += 1;
        }

        Vec2::ZERO
    }

    fn update(&mut self, dt: f32, snapshot: &[Boid]) {
        let mut pos_avg = Vec2::ZERO;
        let mut vel_avg = Vec2::ZERO;
        let mut close_d = Vec2::ZERO;
        self.neighbors = 0;

        for boid in snapshot {
            self.do_compute(boid, &mut pos_avg, &mut vel_avg, &mut close_d);
        }

        let mut acceleration = Vec2::ZERO;

        if close_d.length_squared() > 0.0 {
            acceleration += close_d.normalize() * SEPARATION_WEIGHT;
        }

        if self.neighbors > 0 {
            pos_avg /= self.neighbors as f32;
            vel_avg /= self.neighbors as f32;

            let cohesion = (pos_avg - self.position).normalize() * COHESION_WEIGHT;
            let alignment = (vel_avg - self.velocity).normalize() * ALIGNMENT_WEIGHT;

            acceleration += cohesion.clamp_length(0.0, MAX_FORCE);
            acceleration += alignment.clamp_length(0.0, MAX_FORCE);
        }

        self.velocity += acceleration;
        self.velocity = self.velocity.clamp_length(MIN_SPEED, MAX_SPEED);
        self.position += self.velocity * dt * SIM_SPEED;
        self.draw();
        self.wrap_screen();
    }

    fn wrap_screen(&mut self) {
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

    for _ in 0..BOIDS {
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
        }
        next_frame().await;
    }
}
