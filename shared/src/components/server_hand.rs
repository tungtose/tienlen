use bevy_ecs::prelude::Component;
use naia_bevy_shared::{Property, Replicate};

#[derive(Replicate, Component)]
pub struct ServerHand {
    pub cards: Property<String>,
}

impl ServerHand {
    pub fn new(cards: String) -> Self {
        Self::new_complete(cards)
    }
}
