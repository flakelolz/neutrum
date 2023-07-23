#![allow(unused_variables)]
use crate::{
    finite_state_machine::{
        state_machine::*,
        state_transitions::{common_attack_transitions, common_jump_transitions},
    },
    math::screen_to_world,
    GROUND,
};

use super::{
    state_context::StateContext,
    state_transitions::{common_to_idle_transitions, common_transitions},
};

const WALK_SPEED: i32 = 7000;

fn ground_collision(context: &mut StateContext) -> bool {
    if context.physics.position.y >= screen_to_world(GROUND) {
        context.physics.position.y = screen_to_world(GROUND);
        context.physics.velocity.y = 0;
        context.physics.acceleration.y = 0;
        context.transition_to_state(StateID::Standing);

        return true;
    }

    return false;
}

pub struct Standing;
impl Standing {
    pub fn on_enter(context: &mut StateContext) {
        println!("Standing ENTER");
    }

    pub fn on_update(context: &mut StateContext) {
        // println!("Standing on_update!!!");
        context.physics.velocity.x = 0;
        context.physics.velocity.y = 0;
        if common_transitions(context) {
            // println!("!");
            // Goes to a state depending on the input pressed
        }
    }

    pub fn on_exit(context: &mut StateContext) {
        println!("Standing EXIT");
        println!("Transition to: {:?}", context.next_state);
    }
}

pub struct Crouching;
impl Crouching {
    pub fn on_enter(context: &mut StateContext) {
        println!("Crouching ENTER");
    }

    pub fn on_update(context: &mut StateContext) {
        println!("Crouching UPDATE");
    }

    pub fn on_exit(context: &mut StateContext) {
        println!("Crouching EXIT");
        println!("Transition to: {:?}", context.next_state);
    }
}

pub struct WalkingForwards;
impl WalkingForwards {
    pub fn on_enter(context: &mut StateContext) {
        println!("WalkingForwards ENTER");
    }
    pub fn on_update(context: &mut StateContext) {
        // println!("WalkingForward on_update");

        if common_attack_transitions(context) {
            // Transitions to attack
        } else if common_jump_transitions(context) {
            // Transitions to jump
        } else if context.inputs.back {
            context.transition_to_state(StateID::WalkingBackwards);
            return;
        }

        context.physics.velocity.x = WALK_SPEED;

        if !context.inputs.forward {
            context.transition_to_state(StateID::Standing);
        }
    }
    pub fn on_exit(context: &mut StateContext) {
        println!("WalkingForwards EXIT");
        println!("Transition to: {:?}", context.next_state);
    }
}

pub struct WalkingBackwards;
impl WalkingBackwards {
    pub fn on_enter(context: &mut StateContext) {
        println!("WalkingBackwards ENTER");
    }
    pub fn on_update(context: &mut StateContext) {
        // println!("WalkingBackward on_update");
        if common_attack_transitions(context) {
            // Transitions to attack
        } else if common_jump_transitions(context) {
            // Transitions to jump
        } else if context.inputs.forward {
            context.transition_to_state(StateID::WalkingForwards);
            return;
        }

        context.physics.velocity.x = -WALK_SPEED;

        if !context.inputs.back {
            context.transition_to_state(StateID::Standing);
        }
    }
    pub fn on_exit(context: &mut StateContext) {
        println!("WalkingBackwards EXIT");
        println!("Transition to: {:?}", context.next_state);
    }
}

pub struct Jump;
impl Jump {
    pub fn on_enter(context: &mut StateContext) {
        println!("Jump ENTER");
        if context.physics.position.y <= screen_to_world(GROUND) {
            context.physics.velocity.y = -22000;
        }

        context.physics.acceleration.y = 900;
    }
    pub fn on_update(context: &mut StateContext) {
        if ground_collision(context) {
            return;
        } else if context.inputs.a {
            context.transition_to_state(StateID::AirAttack);
        }
        // println!("{}", context.physics.velocity.y);
    }
    pub fn on_exit(context: &mut StateContext) {
        println!("Jump EXIT");
        println!("Transition to: {:?}", context.next_state);
    }
}

pub struct Attack;
impl Attack {
    pub fn on_enter(context: &mut StateContext) {
        println!("Attack ENTER");
    }

    pub fn on_update(context: &mut StateContext) {
        if context.physics.velocity.x > 0 {
            context.physics.velocity.x -= 1000;
        }
        if context.physics.velocity.x < 0 {
            context.physics.velocity.x += 1000;
        }

        if context.timeline.frames_elapsed >= context.state_duration {
            context.transition_to_state(StateID::Standing);
        }
    }

    pub fn on_exit(context: &mut StateContext) {
        println!("duration: {}", context.state_duration);
        println!("Attack EXIT");
        println!("Transition to: {:?}", context.next_state);
    }
}

pub struct AirAttack;
impl AirAttack {
    pub fn on_enter(context: &mut StateContext) {
        println!("AirAttack ENTER");
    }
    pub fn on_update(context: &mut StateContext) {
        // println!("AirAttack on_update");

        if ground_collision(context) {
            return;
        }
    }
    pub fn on_exit(context: &mut StateContext) {
        println!("AirAttack EXIT");
        println!("Transition to: {:?}", context.next_state);
    }
}

pub struct Reaction;
impl Reaction {
    pub fn on_enter(context: &mut StateContext) {
        println!("Reaction ENTER");
    }
    pub fn on_update(context: &mut StateContext) {
        // println!("Reaction on_update");
    }
    pub fn on_exit(context: &mut StateContext) {
        println!("Reaction EXIT");
    }
}

pub struct GuardReaction;
impl GuardReaction {
    pub fn on_enter(context: &mut StateContext) {
        println!("GuardReaction ENTER");
    }
    pub fn on_update(context: &mut StateContext) {
        println!("GuardReaction UPDATE");
    }
    pub fn on_exit(context: &mut StateContext) {
        println!("GuardReaction EXIT");
    }
}
