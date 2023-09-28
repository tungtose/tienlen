use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use bevy_ecs::{entity::Entity, prelude::Resource};

use bevy_log::info;
use naia_bevy_demo_shared::{components::hand::Hand, messages::Counter};
use naia_bevy_server::{RoomKey, UserKey};

#[derive(Clone)]
pub struct PlayerData {
    pub name: String,
    pub pos: usize,
    pub active: bool,
    pub cards: String,
    pub entity: Entity,
    pub score: u32,
    pub user_key: UserKey,
}

#[derive(Clone)]
pub struct PlayerMap(pub HashMap<UserKey, PlayerData>);

impl Debug for PlayerData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PlayerData")
            .field("pos", &self.pos)
            .field("active", &self.active)
            .field("cards", &self.cards)
            .field("entity", &self.entity)
            .field("score", &self.score)
            .finish()
    }
}

impl PlayerMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn update_cards(&mut self, user_key: &UserKey, cards: String) {
        self.0.get_mut(user_key).unwrap().cards = cards;
    }

    pub fn update_active_player(&mut self, pos: usize) {
        for (_, p) in self.0.iter_mut() {
            p.active = p.pos == pos;
        }
    }

    pub fn update_score(&mut self, user_key: &UserKey, score: u32) {
        self.0.get_mut(user_key).unwrap().score += score;
    }

    pub fn debug(&self) {
        for (_, p) in self.0.iter() {
            info!("----------- PLAYER MAP ---------------");
            info!("{:?}", p);
        }
    }
}

impl Global {
    pub fn new_match(&mut self) {
        self.time = 0.;
        self.table.clear();
        self.cur_active_pos = 0;
        self.leader_turn = true;
    }
}

#[derive(Resource)]
pub struct Global {
    pub counter: Counter,
    pub time: f32,
    pub table: VecDeque<Hand>,
    pub leader_turn: bool,
    pub users_map: HashMap<UserKey, Entity>,
    pub players_map: PlayerMap,
    pub total_player: usize,
    pub cur_active_pos: usize,
    pub main_room_key: RoomKey,
    pub user_to_square_map: HashMap<UserKey, Entity>,
    pub user_to_cursor_map: HashMap<UserKey, Entity>,
    pub client_to_server_cursor_map: HashMap<Entity, Entity>,
    pub square_to_user_map: HashMap<Entity, UserKey>,
}
