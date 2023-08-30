use bevy::prelude::{Query, ResMut, Vec2, With, Without};

use naia_bevy_demo_shared::components::Player;

use crate::{components::LocalPlayer, resources::Global};

pub fn sync_main_player(
    main_player_q: Query<&Player, With<LocalPlayer>>,
    mut global: ResMut<Global>,
) {
    let Ok(player) = main_player_q.get_single() else {
        return;
    };

    global.game.local_player.name = player.name.to_string();
    global.game.local_player.pos = *player.pos;
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
                global.game.player_1.draw_pos = Vec2::new(-335., 35.);
            }
            2 => {
                global.game.player_2.name = player.name.to_string();
                global.game.player_2.is_join = true;
                global.game.player_2.draw_pos = Vec2::new(0., 210.);
            }
            3 => {
                global.game.player_3.name = player.name.to_string();
                global.game.player_3.is_join = true;
                global.game.player_3.draw_pos = Vec2::new(335., 35.);
            }
            _ => unreachable!(),
        }
    }
}
