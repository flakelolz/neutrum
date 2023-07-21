use crate::{
    finite_state_machine::{state_context::StateContext, state_machine::StateMachineProcessor},
    math::IntVector2D,
};

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Copy, Clone)]
pub struct StateComponent {
    pub context: StateContext,
    pub processor: StateMachineProcessor,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct InputComponent {
    pub up: bool,
    pub down: bool,
    pub back: bool,
    pub forward: bool,
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub k: bool,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PhysicsComponent {
    pub position: IntVector2D,
    pub velocity: IntVector2D,
    pub acceleration: IntVector2D,
    pub facing_left: bool,
    pub facing_opponent: bool,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct Hitbox {
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct HitboxGroup {
    pub start_frame: i32,
    pub duration: i32,
    pub hitboxes: Vec<Hitbox>,
}

impl HitboxGroup {
    pub fn is_active_on_frame(&self, frame: i32) -> bool {
        (frame >= self.start_frame) && (frame < (self.start_frame + self.duration))
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct TimelineComponent {
    pub frames_elapsed: i32,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ReactionComponent {
    pub hitstun: i32,
    pub guardstud: i32,
    pub hitstop: i32,
    pub knockback: i32,
    pub attack_has_hit: bool,
}

#[allow(dead_code)]
#[derive(Default, Debug, Clone, Copy)]
pub enum JumpFlags {
    #[default]
    None,
    JumpForward,
    JumpBack,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct ActionFlagsComponent {
    pub jump_flags: JumpFlags,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct HitEvent {
    pub attacker_id: usize,
    pub defender_id: usize,
    pub hitstun: i32,
    pub guardstun: i32,
    pub hitstop: i32,
    pub knockback: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_frames() {
        let hitbox_group = HitboxGroup {
            start_frame: 2,
            duration: 5,
            ..Default::default()
        };

        assert!(!hitbox_group.is_active_on_frame(1));
        assert!(hitbox_group.is_active_on_frame(2));
        assert!(hitbox_group.is_active_on_frame(6));
        assert!(!hitbox_group.is_active_on_frame(7));

        dbg!(hitbox_group);
    }

    #[test]
    fn insert_hitbox() {
        let mut group = HitboxGroup::default();
        let hitbox = Hitbox {
            top: 200,
            left: 300,
            right: 1000,
            bottom: 800,
        };

        assert!(group.hitboxes.is_empty());

        group.hitboxes.push(hitbox);

        assert_eq!(group.hitboxes.len(), 1);
        assert_eq!(group.hitboxes[0].left, hitbox.left);
        assert_eq!(group.hitboxes[0].right, 1000);

        dbg!(group);
    }
}
