use super::{
    state_context::StateContext,
    state_machine::{StateID, StateMachineProcessor},
};

pub fn handle_transition(state_machine: &mut StateMachineProcessor, context: &mut StateContext) {
    let current_state_callback =
        state_machine.registry.common_states[state_machine.current_state as usize];

    if context.transition {
        (current_state_callback.on_exit)(context);

        let next_state = state_machine.registry.common_states[context.next_state as usize];

        (next_state.on_enter)(context);

        context.transition = false;

        state_machine.current_state = context.next_state;
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
