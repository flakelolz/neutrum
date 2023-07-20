use crate::configs::input_config::PlayerInput;
use crate::game_state::GameState;
use raylib::RaylibHandle;

pub fn update_inputs(
    rl: &mut RaylibHandle,
    game_state: &mut GameState,
    input_config: &PlayerInput,
) {
    for entity in 0..game_state.entity_count {
        let inputs = &mut game_state.state[entity].context.inputs;
        let i = entity as i32;

        inputs.forward = rl.is_key_down(input_config.keybard[entity].forward)
            || rl.is_gamepad_button_down(i, input_config.gamepad[entity].forward);

        inputs.back = rl.is_key_down(input_config.keybard[entity].back)
            || rl.is_gamepad_button_down(i, input_config.gamepad[entity].back);
        inputs.a = rl.is_key_down(input_config.keybard[entity].a)
            || rl.is_gamepad_button_down(i, input_config.gamepad[entity].a);
    }
}
