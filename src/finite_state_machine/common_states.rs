#![allow(unused)]
use crate::finite_state_machine::state_machine::*;

use super::{state_context::StateContext, state_transitions::common_transitions};

pub struct Standing;
impl Standing {
    pub fn on_enter(context: &mut StateContext) {
        println!("Standing on_enter!");
    }

    pub fn on_update(context: &mut StateContext) {
        // println!("Standing on_update!!!");
        context.physics.velocity.x = 0;
        if common_transitions(context) {
            println!("!");
        }
    }

    pub fn on_exit(context: &mut StateContext) {
        println!("Standing::on_exit()");
        println!("Transition to: {:?}", context.next_state);
    }
}

pub struct WalkingForward;
impl WalkingForward {
    pub fn on_enter(context: &mut StateContext) {
        println!("WalkingForward on_enter");
    }
    pub fn on_update(context: &mut StateContext) {
        println!("WalkingForward on_update");
    }
    pub fn on_exit(context: &mut StateContext) {
        println!("WalkingForward on_exit");
    }
}

pub struct WalkingBackward;
impl WalkingBackward {
    pub fn on_enter(context: &mut StateContext) {
        println!("WalkingBackward on_enter");
    }
    pub fn on_update(context: &mut StateContext) {
        println!("WalkingBackward on_update");
    }
    pub fn on_exit(context: &mut StateContext) {
        println!("WalkingBackward on_exit");
    }
}

pub struct Crouching;
impl Crouching {
    pub fn on_enter(context: &mut StateContext) {
        println!("Crouching on_enter");
    }

    pub fn on_update(context: &mut StateContext) {
        println!("Crouching on_update");
    }

    pub fn on_exit(context: &mut StateContext) {
        println!("Crouching on_exit");
    }
}

pub struct Attack;
impl Attack {
    pub fn on_enter(context: &mut StateContext) {
        println!("Attack on_enter");
    }

    pub fn on_update(context: &mut StateContext) {
        println!("Attack on_update");
    }

    pub fn on_exit(context: &mut StateContext) {
        println!("Attack on_exit");
    }
}
