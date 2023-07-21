use std::collections::HashMap;

use crate::{character_data::CharacterProperties, game_state::GameData};

use super::{
    common_states::*, state_context::StateContext, state_registry::StateRegistry,
    state_transitions::handle_transition,
};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
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

impl StateID {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Standing => "standing",
            Self::Crouching => "crouching",
            Self::WalkingForward => "walking_forward",
            Self::WalkingBackward => "walking_backward",
            Self::Jump => "jump",
            Self::Attack => "attack",
            Self::Reaction => "reaction",
            Self::GuardReaction => "guard_reaction",
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct StateCallbacks {
    pub on_enter: fn(&mut StateContext),
    pub on_update: fn(&mut StateContext),
    pub on_exit: fn(&mut StateContext),
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

#[derive(Default, Debug, Clone, Copy)]
pub struct StateMachineProcessor {
    pub registry: StateRegistry,
    pub current_state: StateID,
}

impl StateMachineProcessor {
    pub fn update_state(
        &mut self,
        context: &mut StateContext,
        action_data: &CharacterProperties,
        action_map: &HashMap<String, usize>,
    ) {
        let current_state = self.registry.common_states[self.current_state as usize];

        (current_state.on_update)(context);

        handle_transition(self, context, action_data, &action_map);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_name() {
        let fsm = StateMachineProcessor::default();
        assert_eq!(fsm.current_state, StateID::Standing);
        assert_eq!(fsm.current_state.get_name(), "standing");
        assert_eq!(StateID::Reaction.get_name(), "reaction");
    }
}
