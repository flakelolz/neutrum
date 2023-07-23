use crate::game_state::GameState;

pub fn update_physics(game_state: &mut GameState) {
    for entity in 0..game_state.entity_count {
        // Only update physics where there is no hitstop
        if game_state.reaction_components[entity].hitstop <= 0 {
            let player = &mut game_state.state[entity].context.physics;
            // Move position based on current velocity
            player.position = player.position + player.velocity;
            player.velocity = player.velocity + player.acceleration;
        }
    }
}
