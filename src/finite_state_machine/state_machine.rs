use std::collections::HashMap;

use crate::character_data::CharacterProperties;

use super::{
    common_states::*, state_context::StateContext, state_registry::StateRegistry,
    state_transitions::handle_transition,
};

#[derive(Default, Debug, Copy, Clone, PartialEq)]
pub enum StateID {
    Standing,
    Crouching,
    WalkingForwards,
    WalkingBackwards,
    Jump,
    Attack,
    Reaction,
    GuardReaction,
    #[default]
    Nothing,
}
// TODO: separate jump into jump_start, jump_apex and jump_end

impl StateID {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::Standing => "Standing",
            Self::Crouching => "Crouching",
            Self::WalkingForwards => "WalkingForwards",
            Self::WalkingBackwards => "WalkingBackwards",
            Self::Jump => "Jump",
            Self::Attack => "Attack",
            Self::Reaction => "Reaction",
            Self::GuardReaction => "GuardReaction",
            Self::Nothing => "Nothing",
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

#[derive(Debug, Clone, Copy)]
pub struct StateMachineProcessor {
    pub registry: StateRegistry,
    pub current_state: StateID,
}

impl Default for StateMachineProcessor {
    fn default() -> Self {
        Self {
            registry: StateRegistry::default(),
            current_state: StateID::Standing,
        }
    }
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
        assert_eq!(fsm.current_state.get_name(), "Standing");
        assert_eq!(StateID::Reaction.get_name(), "Reaction");
    }
}
