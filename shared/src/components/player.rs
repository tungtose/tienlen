use bevy_ecs::prelude::Component;
use naia_bevy_shared::{Property, Replicate};

#[derive(Component, Replicate)]
pub struct Host;

#[derive(Component, Replicate)]
pub struct Player {
    pub pos: Property<usize>,
}

impl Player {
    pub fn new(pos: usize) -> Self {
        Self::new_complete(pos)
    }
}

#[derive(Component)]
pub struct Ready;
