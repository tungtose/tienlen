use bevy::prelude::*;
use naia_bevy_demo_shared::components::{player::Host, table::Table, Player};

use crate::resources::Global;

pub fn _playable(player_q: Query<&Player>, host_q: Query<&Host>, global: Res<Global>) {
    let total_player_num = player_q.iter().len();

    if let Some(entity) = global.player_entity {
        if let Ok(_host) = host_q.get(entity) {
            info!("THIS PLAY IS HOST!!!");
        }
    }
    info!("TOTAL PLAYER: {}", total_player_num);
}

pub fn test(table_q: Query<&Table>) {
    for table in table_q.iter() {
        info!("GOT Table: {}", table.cards.to_string());
    }
}
