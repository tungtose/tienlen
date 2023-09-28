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
    pub ready: Property<bool>,
}

impl Player {
    pub fn new(pos: usize, name: &str) -> Self {
        let mut active = false;
        if pos == 0 {
            active = true;
        }

        Self::new_complete(pos, active, 0, name.to_string(), false)
    }

    pub fn name(&self) -> String {
        self.name.clone().to_string()
    }
}

#[derive(Component)]
pub struct Ready;

#[derive(Component, Replicate)]
pub struct Active;
