use crate::game_state::GameState;
use raylib::prelude::*;

pub fn show_debug_stats(d: &mut RaylibDrawHandle, game_state: &mut GameState) {
    let frame_count = &game_state.frame_count.to_string();
    d.draw_text(frame_count, 10, 10, 10, Color::WHITE);

    // Show position
    let gp = &game_state.state[0].context.physics.position.x.to_string();
    let gp_x = &game_state.state[0].context.physics.position.x;
    d.draw_text(gp, *gp_x, 25, 10, Color::WHITE);
    // Show input
    if game_state.state[0].context.inputs.a {
        d.draw_text("A", 30, 40, 20, Color::GREEN);
    }

    // move_player(game_state);
}

fn move_player(game_state: &mut GameState) {
    let player = &mut game_state.state[0].context;
    if player.inputs.forward {
        player.physics.velocity.x = 1;
    } else if player.inputs.back {
        player.physics.velocity.x = -1;
    } else {
        player.physics.velocity.x = 0;
    }
}
