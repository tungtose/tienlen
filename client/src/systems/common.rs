use bevy::prelude::*;
use naia_bevy_demo_shared::components::{player::Host, Player};

use crate::resources::Global;

// pub fn playable(commands: Commands, player_q: Query<&Player>) -> bool {
//     let total_player_num = player_q.iter().len();
//
//     let playable = {
//         if total_player_num > 1 {
//             return true;
//         }
//         false
//     };
//
//     playable
// }

// pub fn playable(commands: Commands, player_q: Query<&Player>) {
//     let total_player_num = player_q.iter().len();
//
//     let playable = {
//         if total_player_num > 1 {
//             return true;
//         }
//         false
//     };
//
//     info!("CAN START? {}", playable);
// }

pub fn playable(
    // commands: Commands,
    player_q: Query<&Player>,
    host_q: Query<&Host>,
    global: Res<Global>,
) {
    let total_player_num = player_q.iter().len();

    if let Some(entity) = global.player_entity {
        if let Ok(_host) = host_q.get(entity) {
            info!("THIS PLAY IS HOST!!!");
        }
    }

    // let playable = {
    //     if total_player_num > 1 {
    //         true
    //     }
    //     false
    // };

    info!("TOTAL PLAYER: {}", total_player_num);
}
