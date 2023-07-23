use std::ops::Add;

use crate::{components::{Hitbox, HitboxGroup}, character_data::ActionProperties};

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

// Screen to world conversion
pub fn screen_to_world(coordinate: i32) -> i32 {
    coordinate * 1000
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

pub fn get_translated_active_hitboxes(
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

#[allow(dead_code)]
pub fn get_vulnerable_hitboxes(
    hitbox_pool: &mut [Hitbox],
    action: &ActionProperties,
    frame: i32,
    position: IntVector2D,
) -> usize {
    let mut pool_index: usize = 0;

    // Find all active hitboxes
    for hitbox_group in action.vulnerable_hitbox_groups.iter() {
        if hitbox_group.is_active_on_frame(frame) {
            for hitbox in hitbox_group.hitboxes.iter() {
                assert!(pool_index <= hitbox_pool.len());

                // If we exceed the hitbox pool size, return the size of the hitbox pool and write no more hitboxes.
                if pool_index >= hitbox_pool.len() {
                    return hitbox_pool.len();
                }

                // Translate the hitbox by the character position.
                hitbox_pool[pool_index] = Hitbox {
                    top: hitbox.top + position.y,
                    left: hitbox.left + position.x,
                    bottom: hitbox.bottom + position.y,
                    right: hitbox.right + position.x,
                };
                pool_index += 1;
            }
        }
    }
    pool_index
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_intvector2d() {
        let a = IntVector2D { x: 1, y: 2 };
        let b = IntVector2D { x: 3, y: 4 };
        let c = a.add(b);

        assert_eq!(c.x, 4);
        assert_eq!(c.y, 6);

    }
}
