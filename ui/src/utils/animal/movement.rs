use rand::{Rng, RngCore};
use nannou::{glam::Vec2, math::Vec2Angle};


#[derive(Clone)]
pub struct Movement {
    velocity: Vec2,
    position: Vec2,
}

impl Movement {
    pub fn new(random: &mut dyn RngCore) -> Self {
        Self {
            velocity: Vec2::new(random.gen_range(-0.01..0.01), random.gen_range(-0.01..0.01)),
            position: Vec2::new(random.gen_range(-1.0..1.0), random.gen_range(-1.0..1.0)),
        }
    }

    pub fn get_position(&self) -> &Vec2 {
        &self.position
    }

    pub fn get_angle(&self) -> f32 {
        self.velocity.angle().to_degrees()
    }

    pub fn update(&mut self, delta_time: f32, effect: f32) {
        self.velocity += effect * delta_time;
        self.position += self.velocity * delta_time;
        if self.position.x > 1.0 || self.position.x < -1.0 {
            self.velocity.x = -self.velocity.x;
        }
        if self.position.y > 1.0 || self.position.y < -1.0 {
            self.velocity.y = -self.velocity.y;
        }
    }
}
