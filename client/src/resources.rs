use std::{collections::BTreeMap, default::Default};

use bevy::prelude::{Entity, Resource, Vec2};

use naia_bevy_client::CommandHistory;
use naia_bevy_demo_shared::{components::card::Card, messages::KeyCommand};

pub struct OwnedEntity {
    pub confirmed: Entity,
    pub predicted: Entity,
}

#[derive(Resource)]
pub struct Global {
    pub player_name: String,
    pub player_entity: Option<Entity>,
    pub player_cards: BTreeMap<usize, Card>,
    pub owned_entity: Option<OwnedEntity>,
    pub queued_command: Option<KeyCommand>,
    pub command_history: CommandHistory<KeyCommand>,
    pub game: Game,
}

#[derive(Resource, Debug)]
pub struct Game {
    pub player_1: Player,
    pub player_2: Player,
    pub player_3: Player,
    pub local_player: LocalPlayer,
    pub table_cards: String,
    pub active_player_pos: i32,
    pub timer: String,
}

#[derive(Default, Debug)]
pub struct Player {
    pub name: String,
    pub score: u32,
    pub in_turn: bool,
    pub is_join: bool,
    pub draw_pos: Vec2,
    pub pos: i32,
    pub is_drawed: bool,
}

impl Player {
    pub fn with_pos(pos: usize) -> Self {
        Self {
            draw_pos: Vec2::default(),
            name: pos.to_string(),
            score: 0,
            pos: pos as i32,
            in_turn: false,
            is_join: false,
            is_drawed: false,
        }
    }
}

#[derive(Default, Debug)]
pub struct LocalPlayer {
    pub name: String,
    pub cards: BTreeMap<usize, Card>,
    pub score: u32,
    pub is_join: bool,
    pub in_turn: bool,
    pub is_drawed: bool,
    pub pos: i32,
}

impl Default for Game {
    fn default() -> Self {
        let p1 = Player::with_pos(1);
        let p2 = Player::with_pos(2);
        let p3 = Player::with_pos(3);

        Self {
            player_1: p1,
            player_2: p2,
            player_3: p3,
            active_player_pos: 0,
            table_cards: String::new(),
            local_player: Default::default(),
            timer: "0".to_string(),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Global {
    fn default() -> Self {
        Self {
            player_name: String::new(),
            player_entity: None,
            player_cards: BTreeMap::new(),
            owned_entity: None,
            queued_command: None,
            command_history: CommandHistory::default(),
            game: Game::default(),
        }
    }
}
