use core::error;
use std::collections::HashMap;

use rand::RngCore;

use nannou::math;
use nannou::App;
use nannou::glam::Vec2;
use nannou::wgpu::Texture;
use nannou::geom::Rect;

use crate::utils::animal::brain::Brain;
use crate::utils::animal::specie::Specie;
use crate::utils::animal::movement::Movement;
use crate::utils::animal::appearance::Appearance;


#[derive(Clone)]
pub struct Member {
    brain: Brain,
    specie: Specie,
    movement: Movement,
    appearance: Appearance,
}

impl Member {
    pub fn new(app: &App, random: &mut dyn RngCore, name: &str) -> Self {
        Self {
            brain: Brain::new(),
            specie: Specie::new(name),
            movement: Movement::new(random),
            appearance: Appearance::new(app, name),
        }
    }

    pub fn get_xy(&self, window: &Rect) -> Vec2 {
        let position = self.movement.get_position();
        let x = math::map_range(position.x, -1.0, 1.0, window.left(), window.right());
        let y = math::map_range(position.y, -1.0, 1.0, window.bottom(), window.top());
        Vec2::new(x, y)
    }

    pub fn get_size(&self) -> Vec2 {
        Vec2::new(50.0, 50.0)
    }

    pub fn get_icon(&self) -> &Texture {
        self.appearance.get_icon()
    }

    pub fn get_angle(&self) -> f32 {
        self.movement.get_angle()
    }

    pub fn update(&mut self, delta_time: f32) {
        // let effect = self.brain.get_effect();
        // self.movement.update(delta_time, effect);
        // let mut errors = HashMap::new();
        // errors.insert("velocity".to_string(), vec![effect]);
        // self.brain.update(errors, 0.1);
        // self.brain.update(inputs);
    }
}
