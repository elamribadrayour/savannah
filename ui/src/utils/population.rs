use nannou::App;
use rand::{Rng, RngCore};
use crate::utils::animal::Animal;

#[derive(Clone)]
pub struct Population {
    members: Vec<Animal>,
}

impl Population {
    pub fn new(app: &App, random: &mut dyn RngCore, size: usize) -> Self {
        let mut animals: Vec<Animal> = Vec::new();
        (0..size).for_each(|_| {
            let animal_type = random.gen_range(0..6);
            match animal_type {
                0 => animals.push(Animal::new(app, random, "lion")),
                1 => animals.push(Animal::new(app, random, "tiger")),
                2 => animals.push(Animal::new(app, random, "zebra")),
                3 => animals.push(Animal::new(app, random, "elephant")),
                4 => animals.push(Animal::new(app, random, "eagle")),
                5 => animals.push(Animal::new(app, random, "wolf")),
                _ => animals.push(Animal::new(app, random, "lion")),
            }
        });

        Self { members: animals }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Animal> {
        self.members.iter()
    }

    pub fn update(&mut self, delta_time: f32) {
        for member in &mut self.members {
            member.update( delta_time);
        }
    }
}
