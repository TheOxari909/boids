extern crate rand;
use macroquad::prelude::*;
use rand::prelude::*;

const MAX_SPEED: f32 = 200.0;
const MAX_FORCE: f32 = 100.0;

const SEPARATION_WEIGHT: f32 = 1.5;
const ALIGNMENT_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 1.0;

const SEPARATION_RADIUS: f32 = 70.0;
const PERCEPTION_RADIUS: f32 = 100.0;

const SIM_SPEED: f32 = 1.0;

#[derive(Clone)]
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

    fn calculate_separation(&self, snapshot: &[Boid]) -> Vec2 {
        let mut count = 0;
        let mut steer = Vec2::ZERO;

        for other in snapshot {
            let delta = self.position - other.position;
            let dist = delta.length();

            if dist > 0.0 && dist < SEPARATION_RADIUS {
                steer -= delta.normalize();
                count += 1;
            }
        }

        if count > 0 {
            return steer;
        }

        Vec2::ZERO
    }

    fn calculate_alignment(&self, boids: &[Boid]) -> Vec2 {
        let mut count = 0.0;
        let mut vel_avg = Vec2::ZERO;

        for other in boids {
            let dist = self.position.distance(other.position);

            if dist > SEPARATION_RADIUS && dist < PERCEPTION_RADIUS {
                vel_avg += other.velocity;
                count += 1.0;
            }
        }

        if count > 0.0 {
            vel_avg.x /= count;
            vel_avg.y /= count;
            return vel_avg;
        }

        Vec2::ZERO
    }

    fn calculate_cohesion(&self, boids: &[Boid]) -> Vec2 {
        let mut count = 0.0;
        let mut pos_avg = Vec2::ZERO;

        for other in boids {
            let dist = self.position.distance(other.position);

            if dist > SEPARATION_RADIUS && dist < PERCEPTION_RADIUS {
                pos_avg += other.position;
                count += 1.0;
            }
        }

        if count > 0.0 {
            pos_avg.x /= count;
            pos_avg.y /= count;
            return pos_avg;
        }

        Vec2::ZERO
    }

    fn update(&mut self, dt: f32, snapshot: &[Boid]) {
        let separation = self.calculate_separation(snapshot);
        let alignment = self.calculate_alignment(snapshot);
        let cohesion = self.calculate_cohesion(snapshot);

        // Steering force & acceleration
        let steering = (separation * SEPARATION_WEIGHT)
            + (alignment * ALIGNMENT_WEIGHT)
            + (cohesion * COHESION_WEIGHT);
        self.velocity += steering * dt * SIM_SPEED;
        self.velocity = self.velocity.clamp_length_max(MAX_SPEED);

        // Position math
        self.position += self.velocity * dt * SIM_SPEED;
        self.wrap_screen()
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

    for _ in 0..200 {
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
