#![allow(unused)]

use crate::components::{inputs::InputComponent, physics::PhysicsComponent};

use super::{common_states::Standing, state_transitions::handle_transition, state_context::StateContext};

const MAX_STATES: usize = 8;

#[derive(Default, Debug, Copy, Clone)]
pub enum StateID {
    #[default]
    Standing,
    Crouching,
    WalkingForward,
    WalkingBackward,
    Jump,
    Attack,
    Reaction,
    GuardReaction,
}


#[derive(Debug, Copy, Clone)]
pub struct StateCallbacks {
    pub on_enter: fn(&mut StateContext),
    pub on_exit: fn(&mut StateContext),
    pub on_update: fn(&mut StateContext),
}

impl Default for StateCallbacks {
    fn default() -> Self {
        Self {
            on_enter: Standing::on_enter,
            on_update: Standing::on_update,
            on_exit: Standing::on_exit,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StateRegistry {
    pub common_states: [StateCallbacks; MAX_STATES],
}

impl Default for StateRegistry {
    fn default() -> Self {
        Self {
            common_states: [StateCallbacks::default(); MAX_STATES],
        }
    }
}

impl StateRegistry {
    pub fn register_state(&mut self, state_id: StateID, state_callbacks: StateCallbacks) {
        self.common_states[state_id as usize] = state_callbacks;
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct StateMachineProcessor {
    pub registry: StateRegistry,
    pub current_state: StateID,
}

impl StateMachineProcessor {
    pub fn update_state(&mut self, context: &mut StateContext) {
        let current_state = self.registry.common_states[self.current_state as usize];

        (current_state.on_update)(context);

        handle_transition(self, context);
    }
}
