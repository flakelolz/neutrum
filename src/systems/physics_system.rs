use crate::game_state::GameState;

pub fn update_physics(game_state: &mut GameState) {
    for entity in 0..game_state.entity_count {
        let player = &mut game_state.state[entity].context.physics;
        player.position = player.position + player.velocity;
        player.velocity = player.velocity + player.acceleration;
    }
}
