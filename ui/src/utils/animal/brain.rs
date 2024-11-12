use std::collections::HashMap;
use rand::RngCore;
use crate::utils::animal::neuron::Neuron;

#[derive(Clone)]
pub struct Brain {
    sensory_neurons: HashMap<String, Neuron>,   // Handles perception (sight, sound, food detection)
    motor_neurons: HashMap<String, Neuron>,     // Controls movement and actions
    survival_neurons: HashMap<String, Neuron>,  // Manages instinctual behaviors (danger, predator)
    social_neurons: HashMap<String, Neuron>,    // Social behaviors (pack dynamics, reproduction)
}

impl Brain {
    pub fn new() -> Self {
        Brain { sensory_neurons: HashMap::new(), motor_neurons: HashMap::new(), survival_neurons: HashMap::new(), social_neurons: HashMap::new() }
    }

    pub fn get_effect(&self) -> f32 {
        0.0
    }
}
