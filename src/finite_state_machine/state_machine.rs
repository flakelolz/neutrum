#![allow(unused)]

use crate::components::{inputs::InputComponent, physics::PhysicsComponent};

use super::{common_states::*, state_context::StateContext, state_transitions::handle_transition};

const MAX_STATES: usize = 10;

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
    pub fn init_states(&mut self) {
        let standing_callbacks = StateCallbacks {
            on_enter: Standing::on_enter,
            on_update: Standing::on_update,
            on_exit: Standing::on_exit,
        };

        let walking_forward_callbacks = StateCallbacks {
            on_enter: WalkingForward::on_enter,
            on_update: WalkingForward::on_update,
            on_exit: WalkingForward::on_exit,
        };

        let walking_backward_callbacks = StateCallbacks {
            on_enter: WalkingBackward::on_enter,
            on_update: WalkingBackward::on_update,
            on_exit: WalkingBackward::on_exit,
        };

        let crouching_callbacks = StateCallbacks {
            on_enter: Crouching::on_enter,
            on_update: Crouching::on_update,
            on_exit: Crouching::on_exit,
        };

        let attack_callbacks = StateCallbacks {
            on_enter: Attack::on_enter,
            on_update: Attack::on_update,
            on_exit: Attack::on_exit,
        };

        self.register_state(StateID::Standing, standing_callbacks);
        self.register_state(StateID::Crouching, crouching_callbacks);
        self.register_state(StateID::WalkingForward, walking_forward_callbacks);
        self.register_state(StateID::WalkingBackward, walking_backward_callbacks);
        self.register_state(StateID::Attack, attack_callbacks);
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
