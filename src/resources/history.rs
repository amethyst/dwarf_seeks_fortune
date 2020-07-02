use crate::components::*;

#[derive(Debug)]
pub struct History {
    pub force_key_frame: bool,
    frame_stack: Vec<Frame>,
}

impl Default for History {
    fn default() -> History {
        History {
            force_key_frame: true,
            frame_stack: vec!(),
        }
    }
}

impl History {
    pub fn push_frame(&mut self, frame: Frame) {
        println!("Inserting frame: {:?}", frame);
        self.frame_stack.push(frame);
    }

    pub fn pop_frame(&mut self) -> Option<Frame> {
        self.frame_stack.pop()
    }
}

#[derive(Debug)]
pub struct Frame {
    pub player_position: DiscretePos,
}

impl Frame {
    pub fn new(player_position: DiscretePos) -> Self {
        Frame { player_position }
    }
}

#[derive(Debug, PartialEq)]
pub enum CurrentState {
    Running,
    Rewinding,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::Running
    }
}

#[derive(Debug, Default)]
pub struct Rewind {
    pub cooldown: f32,
}

impl Rewind {
    pub fn is_ready(&self) -> bool {
        self.cooldown.is_sign_negative()
    }
}