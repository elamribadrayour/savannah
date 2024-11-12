use rand::{Rng, RngCore};

#[derive(Clone)]
pub struct Neuron {
    weights: Vec<f32>,
}

impl Neuron {
    pub fn new(random: &mut dyn RngCore) -> Self {
        Self {
            weights: (0..).map(|_| random.gen_range(-1.0..1.0)).collect(),
        }
    }
}

impl Neuron {
    pub fn get_value(&self) -> f32 {
        self.weights.iter().sum()
    }

    pub fn update(&mut self, error: &Vec<f32>, lr: f32) {
        self.weights
            .iter_mut()
            .zip(error.iter())
            .for_each(|(weight, error)| *weight += *error * lr);
    }
}
