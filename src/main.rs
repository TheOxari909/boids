extern crate rand;
use macroquad::prelude::*;
use rand::prelude::*;

const MAX_SPEED: f32 = 250.0;
const MIN_SPEED: f32 = 200.0;
const MAX_FORCE: f32 = 4.0;
const MARGINS: f32 = 0.2;
const TURN_FACTOR: f32 = 4.0;

const BOIDS: usize = 1000;

const SEPARATION_WEIGHT: f32 = 1.7;
const ALIGNMENT_WEIGHT: f32 = 0.8;
const COHESION_WEIGHT: f32 = 1.0;

const SEPARATION_RADIUS: f32 = 25.0;
const PERCEPTION_RADIUS: f32 = 50.0;

const SIM_SPEED: f32 = 1.0;

#[derive(Clone)]
struct Boid {
    position: Vec2,
    velocity: Vec2,
    acceleration: Vec2,
    wander_angle: f32,
}

impl Boid {
    fn new(pos: Vec2, vel: Vec2) -> Self {
        Boid {
            position: pos,
            velocity: vel,
            acceleration: Vec2::ZERO,
            wander_angle: rand::random::<f32>() * std::f32::consts::PI * 2.0,
        }
    }

    fn steer(&self, target_vel: Vec2) -> Vec2 {
        let force = target_vel.normalize_or_zero() * MAX_SPEED - self.velocity;
        force.clamp_length(0.0, MAX_FORCE)
    }

    fn update(&mut self, dt: f32, snapshot: &[Boid]) {
        self.acceleration = Vec2::ZERO;
        let mut separation_sum = Vec2::ZERO;
        let mut alignment_sum = Vec2::ZERO;
        let mut cohesion_sum = Vec2::ZERO;
        let mut count = 0;

        for other in snapshot {
            let dist_sq = self.position.distance_squared(other.position);

            if dist_sq > 0.0 && dist_sq < PERCEPTION_RADIUS * PERCEPTION_RADIUS {
                cohesion_sum += other.position;
                alignment_sum += other.velocity;

                if dist_sq < SEPARATION_RADIUS * SEPARATION_RADIUS {
                    let away = self.position - other.position;
                    separation_sum += away.normalize_or_zero() / dist_sq.sqrt();
                }
                count += 1;
            }
        }

        if count > 0 {
            let align_force = self.steer(alignment_sum);
            self.acceleration += align_force * ALIGNMENT_WEIGHT;

            let center = cohesion_sum / count as f32;
            let seek_force = self.steer(center - self.position);
            self.acceleration += seek_force * COHESION_WEIGHT;

            let sep_force = self.steer(separation_sum);
            self.acceleration += sep_force * SEPARATION_WEIGHT;
        }

        let mut rng = rand::rng();
        self.wander_angle += rng.random_range(-0.5..0.5);

        let wander_vec = Vec2::from_angle(self.wander_angle) * 100.0;
        let mut wander_force = self.steer(wander_vec);
        wander_force *= 0.5;
        self.acceleration += wander_force;

        let turn_force = self.turn_around();
        self.acceleration += turn_force;

        self.velocity += self.acceleration;
        self.velocity = self.velocity.clamp_length(MIN_SPEED, MAX_SPEED);
        self.position += self.velocity * dt * SIM_SPEED;
        self.draw();
    }

    fn turn_around(&mut self) -> Vec2 {
        let mut steering = Vec2::ZERO;
        let margin_x = screen_width() * MARGINS;
        let margin_y = screen_height() * MARGINS;

        if self.position.x < (margin_x) {
            let strength = 1.0 - (self.position.x / margin_x);
            steering.x += strength * TURN_FACTOR;
        } else if self.position.x > screen_width() - margin_x {
            let dist_from_edge = screen_width() - self.position.x;
            let strength = 1.0 - (dist_from_edge / margin_x);
            steering.x -= strength * TURN_FACTOR;
        }

        if self.position.y < margin_y {
            let strength = 1.0 - (self.position.y / margin_y);
            steering.y += strength * TURN_FACTOR;
        } else if self.position.y > screen_height() - margin_y {
            let dist_from_edge = screen_height() - self.position.y;
            let strength = 1.0 - (dist_from_edge / margin_y);
            steering.y -= strength * TURN_FACTOR;
        }

        steering
    }

    fn draw(&self) {
        let angle = self.velocity.y.atan2(self.velocity.x) + std::f32::consts::FRAC_PI_2;
        let rot = Vec2::from_angle(angle);

        let transform = |x: f32, y: f32| -> Vec2 {
            self.position + vec2(x * rot.x - y * rot.y, x * rot.y + y * rot.x)
        };

        let v1 = transform(0.0, -10.0);
        let v2 = transform(-5.0, 5.0);
        let v3 = transform(5.0, 5.0);

        draw_triangle(v1, v2, v3, SKYBLUE);
    }
}

#[macroquad::main("Boids")]
async fn main() {
    let mut rng = rand::rng();

    let mut boids = vec![];

    // set_fullscreen(true);
    request_new_screen_size(1200.0, 700.0);
    for _ in 1..10 {
        next_frame().await;
    }

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

        draw_text(
            format!("FPS: {}", get_fps()).as_str(),
            3.0,
            20.0,
            32.0,
            BLACK,
        );
        draw_text(format!("BOIDS: {}", BOIDS).as_str(), 3.0, 42.0, 32.0, BLACK);
        next_frame().await;
    }
}
