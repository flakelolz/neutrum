use std::collections::HashMap;

use crate::character_data::{find_action, ActionProperties, CharacterProperties};

use super::{
    state_context::StateContext,
    state_machine::{StateID, StateMachineProcessor},
};

pub fn handle_transition(
    state_machine: &mut StateMachineProcessor,
    context: &mut StateContext,
    action_data: &CharacterProperties,
    action_map: &HashMap<String, usize>,
) {
    let current_state_callback =
        state_machine.registry.common_states[state_machine.current_state as usize];
    let state_name = state_machine.current_state.get_name();

    let action = match find_action(action_data, action_map, state_name.to_string()) {
        Some(action) => action,
        None => ActionProperties::default(),
    };

    context.timeline.frames_elapsed += 1;

    if context.timeline.frames_elapsed >= action.duration {
        if action.is_looping {
            context.timeline.frames_elapsed = 0;
        } else if !context.transition {
            if context.physics.position.y > 0 {
                // context.transition_to_state(StateID::Jump);
            } else {
                context.transition_to_state(StateID::Standing);
            }
        }
    }

    if context.transition {
        (current_state_callback.on_exit)(context);

        let next_state = state_machine.registry.common_states[context.next_state as usize];

        (next_state.on_enter)(context);

        context.transition = false;

        state_machine.current_state = context.next_state;

        context.timeline.frames_elapsed = 0;

        let state_name = state_machine.current_state.get_name();

        let action = match find_action(action_data, action_map, state_name.to_string()) {
            Some(action) => action,
            None => ActionProperties::default(),
        };

        context.reaction.attack_has_hit = false;

        context.state_duration = action.duration;
    }
}

pub fn common_attack_transitions(context: &mut StateContext) -> bool {
    if context.inputs.a {
        context.transition_to_state(StateID::Attack);
        return true;
    }
    return false;
}

pub fn common_jump_transitions(context: &mut StateContext) -> bool {
    if context.inputs.up {
        context.transition_to_state(StateID::Jump);
        return true;
    }
    return false;
}

pub fn common_transitions(context: &mut StateContext) -> bool {
    if common_attack_transitions(context) {
        return true;
    } else if common_jump_transitions(context) {
        return true;
    } else if context.inputs.forward {
        context.transition_to_state(StateID::WalkingForward);
        return true;
    } else if context.inputs.back {
        context.transition_to_state(StateID::WalkingBackward);
        return true;
    }
    return false;
}
