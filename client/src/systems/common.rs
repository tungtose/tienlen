use bevy::prelude::*;
use naia_bevy_demo_shared::components::{
    player::{Active, Host},
    // table::Table,
    Player,
};

use crate::resources::Global;

pub fn playable(
    player_q: Query<&Player>,
    host_q: Query<&Host>,
    global: Res<Global>,
    active: Query<&Active>,
) {
    let total_player_num = player_q.iter().len();

    if let Some(entity) = global.player_entity {
        if let Ok(player) = player_q.get(entity) {
            info!("Player POS: {}", *player.pos);
        }

        if let Ok(_host) = host_q.get(entity) {
            info!("THIS PLAY IS HOST!!!");
        }
        if let Ok(_active) = active.get(entity) {
            info!("Player {:?}  IS ACTIVE!!!", entity);
        }
    }
    info!("TOTAL PLAYER: {}", total_player_num);
}
