use bevy::prelude::{Query, ResMut, With, Without};

use naia_bevy_demo_shared::components::{hand::Hand, Counter, Player, Table};

use crate::game::LocalPlayerCards;
use crate::{components::LocalPlayer, resources::Global};

pub fn sync_main_player(
    main_player_q: Query<&Player, With<LocalPlayer>>,
    mut global: ResMut<Global>,
) {
    let Ok(player) = main_player_q.get_single() else {
        return;
    };

    global.game.local_player.name = player.name.to_string();
    global.game.local_player.pos = *player.pos as i32;
    global.game.local_player.is_join = true;
}

pub fn sync_foreign_player(
    player_q: Query<&Player, Without<LocalPlayer>>,
    mut global: ResMut<Global>,
) {
    let main_player_pos = global.game.local_player.pos;
    let is_join = global.game.local_player.is_join;

    if !is_join {
        return;
    }

    for player in player_q.iter() {
        let player_pos = *player.pos;

        let mut player_num: usize = 0;

        match main_player_pos {
            0 => {
                player_num = player_pos;
            }
            1 => {
                if player_pos == 2 {
                    player_num = 1;
                }
                if player_pos == 3 {
                    player_num = 2;
                }
                if player_pos == 0 {
                    player_num = 3;
                }
            }

            2 => {
                if player_pos == 3 {
                    player_num = 1;
                }
                if player_pos == 0 {
                    player_num = 2;
                }
                if player_pos == 1 {
                    player_num = 3;
                }
            }
            3 => {
                if player_pos == 0 {
                    player_num = 1;
                }
                if player_pos == 1 {
                    player_num = 2;
                }

                if player_pos == 2 {
                    player_num = 3;
                }
            }
            _ => unreachable!(),
        }

        match player_num {
            1 => {
                global.game.player_1.name = player.name.to_string();
                global.game.player_1.is_join = true;
                // global.game.player_1.draw_pos = config.p1();
                global.game.player_1.pos = *player.pos as i32;
            }
            2 => {
                global.game.player_2.name = player.name.to_string();
                global.game.player_2.is_join = true;
                // global.game.player_2.draw_pos = config.p2();
                global.game.player_2.pos = *player.pos as i32;
            }
            3 => {
                global.game.player_3.name = player.name.to_string();
                global.game.player_3.is_join = true;
                // global.game.player_3.draw_pos = config.p3();
                global.game.player_3.pos = *player.pos as i32;
            }
            _ => unreachable!(),
        }
    }
}

// pub fn sync_main_player_cards(
//     player_q: Query<&Player, With<LocalPlayer>>,
//     mut local_player_cards_q: Query<&mut LocalPlayerCards>,
// ) {
//     let Ok(player) = player_q.get_single() else {
//         return;
//     };
//
//     let hand_str = player.cards.clone();
//
//     let hand = Hand::from(hand_str);
//
//     if hand.is_empty() {
//         return;
//     }
//
//     let Ok(mut ppp) = local_player_cards_q.get_single_mut() else {
//         return;
//     };
//
//     let cur_value: usize = ppp.0.values().map(|c| c.ordinal()).sum();
//
//     if hand.total_value() != cur_value {
//         ppp.0.clear();
//         for card in hand.cards.as_slice() {
//             ppp.0.insert(card.ordinal(), *card);
//         }
//     }
// }
//
// pub fn sync_table_cards(mut global: ResMut<Global>, server_table_q: Query<&Table>) {
//     let Ok(table_server) = server_table_q.get_single() else {
//         return;
//     };
//
//     let table_cards_str = table_server.cards.to_string();
//
//     if table_cards_str.is_empty() {
//         return;
//     }
//
//     global.game.table_cards = table_server.cards.to_string();
// }

pub fn sync_timer(mut global: ResMut<Global>, timer_q: Query<&Counter>) {
    let Ok(server_timer) = timer_q.get_single() else {
        return;
    };

    global.game.timer = server_timer.as_string();
}

pub fn sync_player(mut global: ResMut<Global>, player_q: Query<&Player>) {
    let game = &mut global.game;
    for player in player_q.iter() {
        if game.player_1.pos == *player.pos as i32 {
            game.player_1.score = *player.score;
        }

        if game.player_2.pos == *player.pos as i32 {
            game.player_2.score = *player.score;
        }

        if game.player_3.pos == *player.pos as i32 {
            game.player_3.score = *player.score;
        }

        if game.local_player.pos == *player.pos as i32 {
            game.local_player.score = *player.score;
        }
    }
}
