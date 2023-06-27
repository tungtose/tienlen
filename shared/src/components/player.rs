use bevy_ecs::prelude::Component;
use naia_bevy_shared::{Property, Replicate};

#[derive(Component, Replicate)]
pub struct Host;

#[derive(Component, Replicate)]
pub struct Player {
    pub pos: Property<usize>,
    pub active: Property<bool>,
}

impl Player {
    pub fn new(pos: usize) -> Self {
        let mut active = false;
        if pos == 0 {
            active = true;
        }

        Self::new_complete(pos, active)
    }
}

#[derive(Component)]
pub struct Ready;

#[derive(Component, Replicate)]
pub struct Active;
