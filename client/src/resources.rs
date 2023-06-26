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

pub struct PlayerCards(HashMap<Entity, Card>);

impl PlayerCards {
    // pub fn new(str: String) -> Self {
    // let hand_str = str
    //     .chars()
    //     .collect::<Vec<char>>()
    //     .chunks(2)
    //     .map(|c| c.iter().collect::<String>())
    //     .collect::<Vec<String>>();
    //
    // let sl: Vec<&str> = hand_str.iter().map(|str| str.as_str()).collect();
    // let rs: HashMap<Entity, Card>= HashMap::new();
    //
    // for card_str in sl {
    //     let card_rs = Card::from_str(card_str);
    //
    //     if let Ok(card) = card_rs {
    //         rs.pushjksh
    //     } else {
    //         info!("SPAWN CARD ERROR: {}", card_str);
    //     }
    // }
    // }
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
}

impl Default for Global {
    fn default() -> Self {
        Self {
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
