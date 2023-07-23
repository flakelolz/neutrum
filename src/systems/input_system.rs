use crate::configs::input_config::PlayerInput;
use crate::game_state::GameState;
use raylib::RaylibHandle;

pub fn update_inputs(
    rl: &mut RaylibHandle,
    game_state: &mut GameState,
    input_config: &PlayerInput,
) {
    for player in 0..input_config.gamepad.len() {
        let inputs = &mut game_state.state[player].context.inputs;
        let i = player as i32;

        inputs.up = rl.is_key_down(input_config.keybard[player].up)
            || rl.is_gamepad_button_down(i, input_config.gamepad[player].up);

        inputs.forward = rl.is_key_down(input_config.keybard[player].forward)
            || rl.is_gamepad_button_down(i, input_config.gamepad[player].forward);

        inputs.back = rl.is_key_down(input_config.keybard[player].back)
            || rl.is_gamepad_button_down(i, input_config.gamepad[player].back);

        inputs.a = rl.is_key_down(input_config.keybard[player].a)
            || rl.is_gamepad_button_down(i, input_config.gamepad[player].a);
    }
}
