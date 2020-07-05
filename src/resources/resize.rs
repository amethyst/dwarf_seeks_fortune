/// This resource is usually set to Idle but becomes Resizing when the window is resized.
/// This triggers a system to recreate the camera entity with the new window dimensions.
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
