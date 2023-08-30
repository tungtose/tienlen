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
    pub active_player_pos: i32,
    pub game: Game,
}

#[derive(Resource, Debug)]
pub struct Game {
    pub player_1: Player,
    pub player_2: Player,
    pub player_3: Player,
    pub local_player: LocalPlayer,
    pub active_player_pos: i32,
}

#[derive(Default, Debug)]
pub struct Player {
    pub name: String,
    pub score: String,
    pub in_turn: bool,
    pub is_join: bool,
    pub draw_pos: Vec2,
    pub is_drawed: bool,
}

impl Player {
    pub fn with_pos(pos: usize) -> Self {
        Self {
            draw_pos: Vec2::default(),
            name: pos.to_string(),
            score: "0".to_string(),
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
    pub score: String,
    pub is_join: bool,
    pub in_turn: bool,
    pub is_drawed: bool,
    pub pos: usize,
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
            local_player: Default::default(),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for Global {
    fn default() -> Self {
        Self {
            player_name: String::new(),
            active_player_pos: 0,
            player_entity: None,
            player_cards: BTreeMap::new(),
            owned_entity: None,
            queued_command: None,
            command_history: CommandHistory::default(),
            game: Game::default(),
        }
    }
}
