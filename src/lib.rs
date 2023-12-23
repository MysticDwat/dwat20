use rand::{Rng, rngs::ThreadRng};

pub struct Die {
    pub sides: u8,
    roller: ThreadRng
}

impl Die {
    pub fn new(sides: u8) -> Self {
        Self {
            sides,
            roller: rand::thread_rng()
        }
    }

    pub fn roll(mut self) -> u8{
        (self.roller.gen::<u8>() % self.sides) + 1
    }
}