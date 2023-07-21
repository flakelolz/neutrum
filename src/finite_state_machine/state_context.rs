use crate::components::{
    ActionFlagsComponent, InputComponent, PhysicsComponent, ReactionComponent, TimelineComponent,
};

use super::state_machine::StateID;

#[derive(Default, Debug, Copy, Clone)]
pub struct StateContext {
    pub transition: bool,
    pub next_state: StateID,
    pub inputs: InputComponent,
    pub physics: PhysicsComponent,
    pub timeline: TimelineComponent,
    pub reaction: ReactionComponent,
    pub action_flags: ActionFlagsComponent,
    pub state_duration: i32,
}

impl StateContext {
    pub fn transition_to_state(&mut self, state_id: StateID) {
        self.transition = true;
        self.next_state = state_id;
    }
}
