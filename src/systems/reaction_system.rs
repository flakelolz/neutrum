use crate::{finite_state_machine::state_machine::StateID, game_state::GameState};

pub fn update_reaction(game_state: &mut GameState) {
    for (entity_index, component) in game_state.reaction_components.iter_mut().enumerate() {
        if component.hitstop > 0 {
            component.hitstop -= 1;
        } else if component.hitstun > 0 {
            component.hitstun -= 1;

            if component.hitstun <= 0 {
                let defender_state = &mut game_state.state[entity_index];

                defender_state.context.transition = true;
                defender_state.context.next_state = StateID::Standing;
            }
        }
    }

    for hit_event in game_state.hit_events.iter() {
        let defender_state = &mut game_state.state[hit_event.defender_id];
        defender_state.context.transition = true;
        defender_state.context.next_state = StateID::Reaction;

        game_state.reaction_components[hit_event.defender_id].hitstun = hit_event.hitstun;
        game_state.reaction_components[hit_event.defender_id].hitstop = hit_event.hitstop;
        game_state.reaction_components[hit_event.attacker_id].hitstop = hit_event.hitstop;
    }
}
