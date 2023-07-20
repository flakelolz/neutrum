use crate::components::{inputs::InputComponent, physics::PhysicsComponent};

use super::state_machine::StateID;

#[derive(Default, Debug, Copy, Clone)]
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
