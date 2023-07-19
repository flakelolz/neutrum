use crate::components::state::StateComponent;

const MAX_ENTITIES: usize = 2;

pub struct GameState {
    pub frame_count: i32,
    pub entity_count: usize,
    pub state: [StateComponent; MAX_ENTITIES],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            frame_count: 0,
            entity_count: MAX_ENTITIES,
            state: [StateComponent::default(); MAX_ENTITIES],
        }
    }
}
