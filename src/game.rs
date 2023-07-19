use raylib::prelude::*;

use crate::{
    debug::debug_stats::show_debug_stats,
    game_state::GameState,
    systems::{
        action_system::update_actions, input_system::update_inputs, physics_system::update_physics,
    },
};
#[allow(unused_variables)]
pub fn game_loop(
    rl: &mut RaylibHandle,
    thread: RaylibThread,
    screen_width: i32,
    screen_height: i32,
) {
    let mut game_state = GameState::default();

    while !rl.window_should_close() {
        // INPUTS
        update_inputs(rl, &mut game_state);

        // Simulation
        update_physics(&mut game_state);
        update_actions(&mut game_state);
        game_state.frame_count += 1;

        // RENDERING
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);

        let p1_pos_x = game_state.state[0].context.physics.position.x;
        let p1_pos_y = game_state.state[0].context.physics.position.y;
        d.draw_circle(p1_pos_x, p1_pos_y, 30.0, Color::BLACK);

        // DEBUG
        show_debug_stats(&mut d, &mut game_state);
    }
}