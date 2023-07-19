use crate::game_state::GameState;
use raylib::prelude::*;
use raylib::RaylibHandle;

pub fn update_inputs(rl: &mut RaylibHandle, game_state: &mut GameState) {
    for entity in 0..game_state.entity_count {
        let inputs = &mut game_state.state[entity].context.inputs;

        inputs.forward = rl.is_key_down(KeyboardKey::KEY_D);
        inputs.back = rl.is_key_down(KeyboardKey::KEY_A);
        inputs.a = rl.is_key_down(KeyboardKey::KEY_J);
    }
}
