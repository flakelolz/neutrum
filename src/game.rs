use raylib::prelude::*;

use crate::{
    configs::input_config::PlayerInput,
    debug::debug_stats::show_debug_stats,
    game_state::GameState,
    rendering::render_player,
    systems::{
        action_system::update_actions, input_system::update_inputs, physics_system::update_physics,
    }, math::IntVector2D,
};
#[allow(unused_variables)]
pub fn game_loop(
    rl: &mut RaylibHandle,
    thread: RaylibThread,
    screen_width: i32,
    screen_height: i32,
) {
    const P1: usize = 0;
    const P2: usize = 1;
    let texture = rl
        .load_texture(&thread, "assets/character.png")
        .expect("texture not found");
    let mut game_state = GameState::default();
    let input_config = PlayerInput::default();
    game_state.state[P1].processor.registry.init_states();
    game_state.state[P2].processor.registry.init_states();

    // Initial position of player
    game_state.state[P1].context.physics.position = IntVector2D {
        x: 300,
        y: 240,
    };
    game_state.state[P2].context.physics.position = IntVector2D {
        x: 800,
        y: 240,
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
        d.clear_background(Color::GRAY);

        render_player(&mut d, &game_state, &texture, P1);

        // DEBUG
        show_debug_stats(&mut d, &mut game_state);
    }
}
