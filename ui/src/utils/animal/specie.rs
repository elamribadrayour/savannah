#[derive(Clone)]
pub enum Specie {
    Zebra,
    Elephant,
    Eagle,
    Wolf,
    Lion,
    Tiger,
}

impl Specie {
    pub fn new(name: &str) -> Self {
        match name {
            "zebra" => Specie::Zebra,
            "elephant" => Specie::Elephant,
            "eagle" => Specie::Eagle,
            "wolf" => Specie::Wolf,
            "lion" => Specie::Lion,
            "tiger" => Specie::Tiger,
            _ => panic!("Invalid specie name: {}", name),
        }
    }
}
