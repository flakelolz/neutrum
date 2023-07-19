#![allow(unused)]
use crate::finite_state_machine::{
    state_context::StateContext, state_machine::StateMachineProcessor,
};

#[derive(Default, Debug, Copy, Clone)]
pub struct StateComponent {
    pub context: StateContext,
    pub processor: StateMachineProcessor,
}
