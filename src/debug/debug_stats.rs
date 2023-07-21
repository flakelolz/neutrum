use crate::{game_state::GameState, math::world_to_screen};
use raylib::prelude::*;

pub fn show_debug_stats(d: &mut RaylibDrawHandle, game_state: &mut GameState, player: usize) {
    show_frame_count(d, game_state);

    show_inputs(d, game_state);

    show_position(d, game_state, player);

    show_state(d, game_state, player);

    // let x = world_to_screen(game_state.state[1].context.physics.position.x) as f32;
    // let y = world_to_screen(game_state.state[1].context.physics.position.y) as f32;
    // d.draw_circle(x as i32, y as i32, 5.0, Color::GREEN);
}

fn show_frame_count(d: &mut RaylibDrawHandle, game_state: &mut GameState) {
    let frame_count = &game_state.frame_count.to_string();
    d.draw_text(frame_count, 10, 10, 10, Color::WHITE);
}

fn show_inputs(d: &mut RaylibDrawHandle, game_state: &mut GameState) {
    // Show input
    if game_state.state[0].context.inputs.a {
        d.draw_text("A", 30, 40, 20, Color::GREEN);
    }
}

fn show_position(d: &mut RaylibDrawHandle, game_state: &mut GameState, player: usize) {
    let color = match player {
        0 => Color::PURPLE,
        _ => Color::GREEN,
    };
    let x_pos = world_to_screen(game_state.state[player].context.physics.position.x) as f32;
    let y_pos = world_to_screen(game_state.state[player].context.physics.position.y) as f32;
    d.draw_circle(x_pos as i32, y_pos as i32, 5.0, color);

    // Show position
    let pos_s = &world_to_screen(game_state.state[player].context.physics.position.x).to_string();
    let pos_i = &world_to_screen(game_state.state[player].context.physics.position.x);
    d.draw_text(pos_s, *pos_i, 25, 10, Color::WHITE);
}

fn show_state(d: &mut RaylibDrawHandle, game_state: &mut GameState, player: usize) {
    let state = game_state.state[player].processor.current_state.get_name();
    let x_pos = world_to_screen(game_state.state[player].context.physics.position.x);
    let y_pos = world_to_screen(game_state.state[player].context.physics.position.y);

    let timeline = game_state.state[player]
        .context
        .timeline
        .frames_elapsed
        .to_string();
    d.draw_text(timeline.as_str(), x_pos, y_pos - 280, 20, Color::WHITE);
    d.draw_text(state, x_pos, y_pos - 250, 20, Color::WHITE);
}
