use crate::{components::{inputs::InputComponent, physics::PhysicsComponent}, math::IntVector2D};

use super::state_machine::StateID;


#[derive(Debug, Copy, Clone)]
pub struct StateContext {
    pub transition: bool,
    pub next_state: StateID,
    pub inputs: InputComponent,
    pub physics: PhysicsComponent,
}

impl StateContext {
    pub fn transition_to_state(&mut self, state_id: StateID) {
        self.transition = true;
        self.next_state = state_id;
    }
}

impl Default for StateContext {
    fn default() -> Self {
        Self {
            transition: false,
            next_state: StateID::Standing,
            inputs: InputComponent::default(),
            physics: PhysicsComponent {
                position: IntVector2D { x: 300, y: 240 },
                ..Default::default()
            }
        }
    }
}
