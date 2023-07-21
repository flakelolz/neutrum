use raylib::prelude::*;

use crate::{
    configs::input_config::PlayerInput,
    debug::debug_stats::show_debug_stats,
    game_state::GameState,
    math::IntVector2D,
    rendering::render_player,
    systems::{
        action_system::update_actions, input_system::update_inputs, physics_system::update_physics,
    },
};
pub fn game_loop(rl: &mut RaylibHandle, thread: RaylibThread) {
    const P1: usize = 0;
    const P2: usize = 1;
    let texture = rl.load_texture(&thread, "assets/sprites/character.png");
    let mut game_state = GameState::default();
    let mut input_config = PlayerInput::default();
    input_config.keybard[P2].set_p2();

    game_state.state[P1].processor.registry.init_states();
    game_state.state[P2].processor.registry.init_states();

    // Initial position of player
    game_state.state[P1].context.physics.position = IntVector2D {
        x: 300000,
        y: 540000,
    };
    game_state.state[P2].context.physics.position = IntVector2D {
        x: 800000,
        y: 540000,
    };

    while !rl.window_should_close() {
        // INPUTS
        update_inputs(rl, &mut game_state, &input_config);

        // SIMULATION
        update_physics(&mut game_state);
        update_actions(&mut game_state);
        game_state.frame_count += 1;

        // RENDERING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        render_player(&mut d, &game_state, &texture, P1);

        // DEBUG
        show_debug_stats(&mut d, &mut game_state);
    }
}
