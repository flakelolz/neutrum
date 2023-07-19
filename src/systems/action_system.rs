use crate::game_state::GameState;

pub fn update_actions(game_state: &mut GameState) {
    for entity in 0..game_state.entity_count {

        game_state.state[entity].processor.update_state(&mut game_state.state[entity].context);
        
    }
}
