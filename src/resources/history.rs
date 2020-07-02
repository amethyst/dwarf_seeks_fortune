use crate::components::*;

#[derive(Debug, Default)]
pub struct History {
    frames: Vec<Frame>,
}

impl History {
    pub fn add_frame(&mut self, frame: Frame) {
        println!("Inserting frame: {:?}", frame);
        self.frames.insert(0, frame);
    }
}

#[derive(Debug)]
pub struct Frame {
    player_position: DiscretePos,
}

impl Frame {
    pub fn new(player_position: DiscretePos) -> Self {
        Frame { player_position }
    }
}