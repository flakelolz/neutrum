use std::ops::Add;

use crate::components::{Hitbox, HitboxGroup};

#[derive(Default, Debug, Clone, Copy)]
pub struct IntVector2D {
    pub x: i32,
    pub y: i32,
}

impl Add for IntVector2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// World to screen conversion
pub fn world_to_screen(coordinate: i32) -> i32 {
    coordinate / 1000
}

pub fn translate_hitbox(hitbox: Hitbox, offset: IntVector2D) -> Hitbox {
    Hitbox {
        left: (hitbox.left + offset.x),
        right: (hitbox.right + offset.x),
        top: (hitbox.top + offset.y),
        bottom: (hitbox.bottom + offset.y),
    }
}

pub fn do_hiboxes_overlap(a: Hitbox, b: Hitbox) -> bool {
    let not_overlapping =
        (a.left > b.right) || (b.left > a.right) || (a.bottom > b.top) || (b.bottom > a.top);

    !not_overlapping
}

fn get_translated_active_hitboxes(
    hitbox_groups: &mut [HitboxGroup],
    hitboxes: &mut [Hitbox],
    offset: IntVector2D,
    frames_elapsed: i32,
) -> usize {
    let mut count: usize = 0;

    for hitbox_group in hitbox_groups.iter() {
        if hitbox_group.is_active_on_frame(frames_elapsed) {
            for hitbox in hitbox_group.hitboxes.iter() {
                let translated_box = translate_hitbox(*hitbox, offset);

                hitboxes[count] = translated_box;
                count += 1;
            }
        }
    }

    count
}
