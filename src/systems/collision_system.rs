use crate::{
    character_data::{find_action, ActionProperties},
    components::HitEvent,
    game_state::GameState,
    math::{do_hiboxes_overlap, get_translated_active_hitboxes}, MAX_ENTITIES,
};

pub fn update_collision(game_state: &mut GameState) {
    let mut atk_count = [0; MAX_ENTITIES];
    let mut vul_count = [0; MAX_ENTITIES];
    for entity in 0..game_state.entity_count {
        let entity_offset = game_state.state[entity].context.physics.position;

        let component = &game_state.state[entity];

        let timeline = game_state.state[entity].context.timeline;

        let current_state = component.processor.current_state;

        // let state = component.processor.registry.common_states[current_state as usize];

        let action_name = current_state.get_name();

        if entity >= game_state.game_data.characters.len() {
            continue;
        }

        let mut action_data = match find_action(
            &game_state.game_data.characters[entity],
            &game_state.game_data.action_maps[entity],
            action_name.to_string(),
        ) {
            Some(action) => action,
            None => ActionProperties::default(),
        };


        {
            let attack = get_translated_active_hitboxes(
                &mut action_data.attack_hitbox_groups,
                &mut game_state.attack_hitbox_scratch[entity],
                entity_offset,
                timeline.frames_elapsed,
            );

            if attack > 0 {
                atk_count[entity] += 1;
            }
        }

        {
            let vulnerable = get_translated_active_hitboxes(
                &mut action_data.vulnerable_hitbox_groups,
                &mut game_state.vulnerable_hitbox_scratch[entity],
                entity_offset,
                timeline.frames_elapsed,
            );

            if vulnerable > 0 {
                vul_count[entity] += 1;
            }
        }
    }

    game_state.hit_events.clear();

    let active_attack_slices = &game_state.attack_hitbox_scratch[..game_state.entity_count];
    let active_vulnerable_slices = &game_state.vulnerable_hitbox_scratch[..game_state.entity_count];

    for (attacker_index, one_entity_attack_boxes) in active_attack_slices.iter().enumerate() {
        if game_state.state[attacker_index]
            .context
            .reaction
            .attack_has_hit
        {
            continue;
        }

        for attack_box in one_entity_attack_boxes[..atk_count[attacker_index]].iter() {
            for (defender_index, one_entity_vulnerable_boxes) in
                active_vulnerable_slices.iter().enumerate()
            {
                if attacker_index == defender_index {
                    continue;
                }

                for vulnerable_box in
                    one_entity_vulnerable_boxes[..vul_count[defender_index]].iter()
                {
                    if do_hiboxes_overlap(*attack_box, *vulnerable_box) {
                        game_state.state[attacker_index]
                            .context
                            .reaction
                            .attack_has_hit = true;

                        println!("Hitboxes Overlap!!");

                        game_state.hit_events.push(HitEvent {
                            attacker_id: attacker_index,
                            defender_id: defender_index,
                            hitstun: 25,
                            guardstun: 10,
                            hitstop: 10,
                            knockback: 5,
                        });
                    }
                }
            }
        }
    }
}
