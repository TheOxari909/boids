extern crate rand;
use macroquad::prelude::*;
use rand::prelude::*;

const MAX_SPEED: f32 = 10.0;
const MAX_FORCE: f32 = 50.0;
const DESIRED_SPEED: f32 = 7.0;
const SEPARATION_WEIGHT: f32 = 15.0;
const ALIGNMENT_WEIGHT: f32 = 1.0;
const COHESION_WEIGHT: f32 = 10.0;
const PERCEPTION_RADIUS: f32 = 75.0;
const SIM_SPEED: f32 = 20.0;

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

    fn calculate_separation(&self, snapshot: &Boid) -> Vec2 {
        let radius2 = PERCEPTION_RADIUS * PERCEPTION_RADIUS;

        let delta = self.position - snapshot.position;
        let dist2 = delta.length_squared();
        if dist2 == 0.0 || dist2 > radius2 {
            return Vec2::ZERO;
        }

        delta.normalize()
    }

    fn calculate_alignment(&self, boids: &[Boid]) -> Vec2 {
        let mut count = 0;
        let mut sum_vel = Vec2::ZERO;

        for boid in boids {
            let dist = self.position.distance(boid.position);
            if dist > 0.0 && dist < PERCEPTION_RADIUS {
                sum_vel += boid.velocity;
                count += 1;
            }
        }
        if count > 0 {
            let average_vel = sum_vel / count as f32;
            return average_vel - self.velocity;
        }

        Vec2::ZERO
    }

    fn calculate_cohesion(&self, boids: &[Boid]) -> Vec2 {
        // Implementation here
        Vec2::new(0.0, 0.0)
    }

    fn update(&mut self, dt: f32, snapshot: &[Boid]) {
        let mut separation = Vec2::ZERO;
        let mut alignment = Vec2::ZERO;
        let mut cohesion = Vec2::ZERO;

        for boid in snapshot {
            if std::ptr::eq(self, boid) {
                continue;
            }
            separation += self.calculate_separation(boid);
        }
        alignment = self.calculate_alignment(snapshot);
        cohesion = self.calculate_cohesion(snapshot);

        // Steering force & acceleration
        let steering = (separation * SEPARATION_WEIGHT)
            + (alignment * ALIGNMENT_WEIGHT)
            + (cohesion * COHESION_WEIGHT);
        self.velocity += steering.clamp_length_max(MAX_FORCE) * dt;

        // Speed limit
        if self.velocity.length() > 0.0 {
            self.velocity = self.velocity.normalize() * DESIRED_SPEED;
        }
        self.velocity = self.velocity.clamp_length_max(MAX_SPEED);

        // Position math
        self.position += self.velocity * dt * SIM_SPEED;

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
