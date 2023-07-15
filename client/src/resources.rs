use std::{collections::BTreeMap, default::Default};

use bevy::prelude::{ColorMaterial, Entity, Handle, Mesh, Resource};

use naia_bevy_client::CommandHistory;
use naia_bevy_demo_shared::{components::card::Card, messages::KeyCommand};

pub struct OwnedEntity {
    pub confirmed: Entity,
    pub predicted: Entity,
}

#[derive(Resource)]
pub struct Global {
    pub player_entity: Option<Entity>,
    pub player_cards: BTreeMap<usize, Card>,
    pub owned_entity: Option<OwnedEntity>,
    pub cursor_entity: Option<Entity>,
    pub queued_command: Option<KeyCommand>,
    pub command_history: CommandHistory<KeyCommand>,
    pub red: Handle<ColorMaterial>,
    pub blue: Handle<ColorMaterial>,
    pub yellow: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub white: Handle<ColorMaterial>,
    pub purple: Handle<ColorMaterial>,
    pub orange: Handle<ColorMaterial>,
    pub aqua: Handle<ColorMaterial>,
    pub circle: Handle<Mesh>,
    pub active_player_pos: i32,
}

#[allow(clippy::derivable_impls)]
impl Default for Global {
    fn default() -> Self {
        Self {
            active_player_pos: 0,
            player_entity: None,
            player_cards: BTreeMap::new(),
            owned_entity: None,
            cursor_entity: None,
            queued_command: None,
            command_history: CommandHistory::default(),
            circle: Handle::default(),
            red: Handle::default(),
            blue: Handle::default(),
            yellow: Handle::default(),
            green: Handle::default(),
            white: Handle::default(),
            purple: Handle::default(),
            orange: Handle::default(),
            aqua: Handle::default(),
        }
    }
}
