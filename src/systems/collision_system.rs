use crate::game_state::GameState;

pub fn update_collision(game_state: &mut GameState) {
    let mut atk_count = [0, 0];
    let mut vul_count = [0, 0];
    for entity in 0..game_state.entity_count {

        let entity_offset = game_state.state[entity].context.physics.position;

        let component = &game_state.state[entity];

        let timeline = game_state.state[entity].context.timeline;

        let current_state = component.processor.current_state;

        let state = component.processor.registry.common_states[current_state as usize];

        let action_name = current_state.get_name();

        // let mut action data = find_action(&game_state.game_data.characters[entity])


    }
}
