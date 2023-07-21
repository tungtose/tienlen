use bevy_ecs::prelude::Component;
use naia_bevy_shared::{Property, Replicate};

#[derive(Component, Replicate)]
pub struct Host;

#[derive(Component, Replicate)]
pub struct Player {
    pub pos: Property<usize>,
    pub active: Property<bool>,
    pub score: Property<u32>,
    pub name: Property<String>,
}

impl Player {
    pub fn new(pos: usize) -> Self {
        let mut active = false;
        if pos == 0 {
            active = true;
        }

        let name = format!("Player {}", pos);

        Self::new_complete(pos, active, 0, name)
    }
}

#[derive(Component)]
pub struct Ready;

#[derive(Component, Replicate)]
pub struct Active;
