use crate::MAX_STATES;

use super::{
    common_states::*,
    state_machine::{StateCallbacks, StateID},
};

#[derive(Default, Debug, Clone, Copy)]
pub struct StateRegistry {
    pub common_states: [StateCallbacks; MAX_STATES],
}

// impl Default for StateRegistry {
//     fn default() -> Self {
//         Self {
//             common_states: [StateCallbacks::default(); MAX_STATES],
//         }
//     }
// }

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
            on_enter: WalkingForwards::on_enter,
            on_update: WalkingForwards::on_update,
            on_exit: WalkingForwards::on_exit,
        };

        let walking_backward_callbacks = StateCallbacks {
            on_enter: WalkingBackwards::on_enter,
            on_update: WalkingBackwards::on_update,
            on_exit: WalkingBackwards::on_exit,
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

        let reaction_callbacks = StateCallbacks {
            on_enter: Reaction::on_enter,
            on_update: Reaction::on_update,
            on_exit: Reaction::on_exit,
        };

        let guard_reaction_callbacks = StateCallbacks {
            on_enter: GuardReaction::on_enter,
            on_update: GuardReaction::on_update,
            on_exit: GuardReaction::on_exit,
        };

        self.register_state(StateID::Standing, standing_callbacks);
        self.register_state(StateID::Crouching, crouching_callbacks);
        self.register_state(StateID::WalkingForwards, walking_forward_callbacks);
        self.register_state(StateID::WalkingBackwards, walking_backward_callbacks);
        self.register_state(StateID::Attack, attack_callbacks);
        self.register_state(StateID::Reaction, reaction_callbacks);
        self.register_state(StateID::GuardReaction, guard_reaction_callbacks);
    }
}
