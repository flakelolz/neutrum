use std::collections::HashMap;

use crate::{
    character_data::{generate_action_map, load_asset, CharacterProperties},
    components::{HitEvent, Hitbox, ReactionComponent, StateComponent},
    MAX_ENTITIES,
};

// #[derive(Default)]
pub struct GameState {
    pub frame_count: i32,
    pub entity_count: usize,
    pub state: [StateComponent; MAX_ENTITIES],
    pub game_data: GameData,
    pub vulnerable_hitbox_scratch: [[Hitbox; 32]; MAX_ENTITIES],
    pub attack_hitbox_scratch: [[Hitbox; 32]; MAX_ENTITIES],
    pub push_hitbox_scratch: [[Hitbox; 32]; MAX_ENTITIES],
    pub hit_events: Vec<HitEvent>,
    pub reaction_components: [ReactionComponent; MAX_ENTITIES],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            frame_count: 0,
            entity_count: MAX_ENTITIES,
            state: [StateComponent::default(); MAX_ENTITIES],
            game_data: GameData::init(Characters::TestCharacter, Characters::TestCharacter),
            vulnerable_hitbox_scratch: [[Hitbox::default(); 32]; MAX_ENTITIES],
            attack_hitbox_scratch: [[Hitbox::default(); 32]; MAX_ENTITIES],
            push_hitbox_scratch: [[Hitbox::default(); 32]; MAX_ENTITIES],
            hit_events: Vec::with_capacity(MAX_ENTITIES),
            reaction_components: [ReactionComponent::default(); MAX_ENTITIES],
        }
    }
}

pub enum Characters {
    TestCharacter,
}
impl Characters {
    fn data_path(&self) -> &str {
        match self {
            Characters::TestCharacter => "assets/data/character_data.json",
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct GameData {
    pub characters: Vec<CharacterProperties>,
    pub action_maps: Vec<HashMap<String, usize>>,
}

impl GameData {
    pub fn init(p1: Characters, p2: Characters) -> Self {
        let data1 = load_asset(p1.data_path());
        let data2 = load_asset(p2.data_path());

        let character1 = match data1 {
            Some(character) => character,
            None => {
                eprintln!("Failed to load character data");
                CharacterProperties::default()
            }
        };

        let character2 = match data2 {
            Some(character) => character,
            None => {
                eprintln!("Failed to load character data");
                CharacterProperties::default()
            }
        };

        let actions1 = generate_action_map(&character1);
        let actions2 = generate_action_map(&character2);

        Self {
            characters: vec![character1, character2],
            action_maps: vec![actions1, actions2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn character_data() {
        let data1 = load_asset("assets/data/character_data.json");

        let mut game_data = GameData::default();

        assert!(game_data.characters.is_empty());
        assert!(game_data.action_maps.is_empty());

        if let Some(data) = data1 {
            game_data.characters.push(data);
        }

        assert_eq!(game_data.characters.len(), 1);

        dbg!(game_data.characters);
    }

    #[test]
    fn init_data() {
        let game_data = GameData::init(Characters::TestCharacter, Characters::TestCharacter);

        assert_eq!(game_data.characters.len(), 2);
        assert_eq!(game_data.action_maps.len(), 2);

        dbg!(game_data);
    }
}
