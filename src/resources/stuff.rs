#[derive(Debug, PartialEq)]
pub enum ResizeState {
    Idle,
    Resizing,
}

impl Default for ResizeState {
    fn default() -> Self {
        ResizeState::Idle
    }
}
