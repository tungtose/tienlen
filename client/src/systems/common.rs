use bevy::prelude::*;
use naia_bevy_demo_shared::components::{player::Host, server_hand::ServerHand, Player};

use crate::{components::LocalPlayer, resources::Global};

pub fn _playable(
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
    info!("TOTAL PLAYER: {}", total_player_num);
}

pub fn _test(_hand_q: Query<&ServerHand, With<LocalPlayer>>) {
    // for hand in hand_q.iter() {
    //     let a = hand.cards.to_string();
    //
    //     info!("GOT HAND: {:?}", a);
    // }
}
