use std::{
    collections::{BTreeMap, HashMap},
    default::Default,
};

use bevy::prelude::{ColorMaterial, Entity, Handle, Mesh, Resource};

use naia_bevy_client::CommandHistory;
use naia_bevy_demo_shared::{components::card::Card, messages::KeyCommand};

pub struct OwnedEntity {
    pub confirmed: Entity,
    pub predicted: Entity,
}

impl OwnedEntity {
    pub fn new(confirmed_entity: Entity, predicted_entity: Entity) -> Self {
        OwnedEntity {
            confirmed: confirmed_entity,
            predicted: predicted_entity,
        }
    }
}

#[derive(Resource)]
pub struct Global {
    pub player_entity: Option<Entity>,
    pub player_cards: BTreeMap<usize, Card>,
    pub owned_entity: Option<OwnedEntity>,
    pub cursor_entity: Option<Entity>,
    // pub all_cards_map: BTreeMap<usize, Card>,
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
}

impl Default for Global {
    fn default() -> Self {
        // let mut all_cards_map: BTreeMap<usize, Card> = BTreeMap::new();
        //
        // Card::all_cards().iter().for_each(|card| {
        //     all_cards_map.insert(card.ordinal(), *card).unwrap();
        // });

        Self {
            player_entity: None,
            player_cards: BTreeMap::new(),
            // all_cards_map,
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
