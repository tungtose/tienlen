use std::collections::{HashMap, VecDeque};

use bevy_ecs::{entity::Entity, prelude::Resource};

use naia_bevy_demo_shared::{
    components::{deck::Deck, hand::Hand},
    messages::Counter,
};
use naia_bevy_server::{RoomKey, UserKey};

#[derive(Resource)]
pub struct Global {
    pub counter: Counter,
    pub deck: Deck,
    pub time: f32,
    pub table: VecDeque<Hand>,
    pub allow_free_combo: bool,
    pub users_map: HashMap<UserKey, Entity>,
    pub players_map: HashMap<usize, Entity>,
    pub total_player: usize,
    pub cur_active_pos: usize,
    pub main_room_key: RoomKey,
    pub user_to_square_map: HashMap<UserKey, Entity>,
    pub user_to_cursor_map: HashMap<UserKey, Entity>,
    pub client_to_server_cursor_map: HashMap<Entity, Entity>,
    pub square_to_user_map: HashMap<Entity, UserKey>,
}
