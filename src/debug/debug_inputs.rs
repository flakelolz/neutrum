use raylib::RaylibHandle;
use raylib::prelude::KeyboardKey;

pub fn update_debug_inputs(rl: &mut RaylibHandle, paused: &mut bool, advance: &mut bool) {
    if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
        *paused = !*paused;
    } else if rl.is_key_pressed(KeyboardKey::KEY_BACKSLASH) {
        *advance = true;
    }
}
