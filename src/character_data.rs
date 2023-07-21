use std::collections::HashMap;

use crate::components::{Hitbox, HitboxGroup};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ActionProperties {
    pub name: String,
    pub duration: i32,
    pub is_looping: bool,
    pub vulnerable_hitbox_groups: Vec<HitboxGroup>,
    pub attack_hitbox_groups: Vec<HitboxGroup>,
    pub push_hitbox_groups: Vec<HitboxGroup>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CharacterProperties {
    pub max_health: i32,
    pub default_pushbox: Hitbox,
    pub actions: Vec<ActionProperties>,
}

pub fn load_asset(path: &str) -> Option<CharacterProperties> {
    match std::fs::File::options().read(true).open(path) {
        Ok(file) => {
            let reader = std::io::BufReader::new(file);
            let character_asset: CharacterProperties =
                serde_json::from_reader(reader).expect("Deserialization failed");

            return Some(character_asset);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return None;
        }
    }

    // let json_reader = std::io::BufReader::new(json_file);

    // Some(character_asset)
}

pub fn generate_action_map(character: &CharacterProperties) -> HashMap<String, usize> {
    let mut action_map = HashMap::new();

    for (index, action) in character.actions.iter().enumerate() {
        action_map.insert(action.name.clone(), index);
    }

    action_map
}

pub fn find_action(
    character: &CharacterProperties,
    action_map: &HashMap<String, usize>,
    action_name: String,
) -> Option<ActionProperties> {
    match action_map.get(&action_name) {
        Some(index) => return Some(character.actions[*index].clone()), // TODO: try to fix clone()
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_properties() {
        let mut group = HitboxGroup::default();

        let hitbox = Hitbox {
            left: 1000,
            right: 2000,
            top: 4000,
            bottom: 5000,
        };

        group.hitboxes.push(hitbox);

        let mut properties = ActionProperties {
            name: "test".to_string(),
            ..Default::default()
        };

        properties.attack_hitbox_groups.push(group);

        assert_eq!(properties.name, "test");
        assert_eq!(properties.attack_hitbox_groups.len(), 1);
        assert_eq!(properties.attack_hitbox_groups[0].hitboxes.len(), 1);
        assert_eq!(properties.attack_hitbox_groups[0].hitboxes[0].top, 4000);
    }

    #[test]
    fn action_map() {
        let character = load_asset("assets/data/character_data.json").expect("load_asset() failed");

        let action_map = &generate_action_map(&character);

        dbg!(action_map);

        let action = find_action(&character, action_map, "Standing".to_string());

        dbg!(action);
    }
}
