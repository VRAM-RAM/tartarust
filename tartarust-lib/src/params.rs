use crate::generation::Pepper;


pub struct TartarusParams {
    pub memory: u32,
    pub iterations: u32,
    pub pepper: Pepper,
}

impl TartarusParams {
    pub fn new(memory: u32, iterations: u32, pepper: Pepper) -> Self {
        Self { memory, iterations, pepper }
    }

    pub fn recommended() -> Self {
        Self {
            memory: 64,
            iterations: 3,
            pepper: Pepper::generate(),
        }
    }

    pub fn test() -> Self {
       Self {
            memory: 12,
            iterations: 1,
            pepper: Pepper::generate(),
       }
    }

    pub fn with_pepper(&mut self, pepper: Pepper) {
        self.pepper = pepper
    }
}

