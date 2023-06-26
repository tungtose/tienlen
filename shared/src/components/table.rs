use bevy_ecs::prelude::Component;
use naia_bevy_shared::{Property, Replicate};

#[derive(Replicate, Component)]
pub struct Table {
    pub cards: Property<String>,
}

impl Default for Table {
    fn default() -> Self {
        Self::new_complete("".to_string())
    }
}

impl Table {
    pub fn new(card_str: String) -> Self {
        Self::new_complete(card_str)
    }
}
