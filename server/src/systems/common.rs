use std::time::Duration;

use bevy_ecs::{
    system::{Commands, Query, Res, ResMut, Resource},
    world::Mut,
};
use bevy_log::info;
use bevy_time::{Time, Timer, TimerMode};
use naia_bevy_demo_shared::{
    channels::GameSystemChannel,
    components::{timer::Counter, turn::Turn, Player},
    messages::UpdateTurn,
};
use naia_bevy_server::Server;

use crate::resources::Global;

#[derive(Resource)]
pub struct CounterConfig {
    timer: Timer,
}

pub fn countdown(
    time: Res<Time>,
    mut config: ResMut<CounterConfig>,
    mut countdown_q: Query<&mut Counter>,
) {
    // tick the timer
    config.timer.tick(time.delta());

    if config.timer.finished() {
        if let Ok(mut counter) = countdown_q.get_single_mut() {
            counter.decr_counter();
        }
    }
}

pub trait PlayerIterator<'a>: Iterator {
    fn current_active_player(&'a mut self) -> &Player;
    fn some_player_not_ready(&'a mut self) -> bool;
}

impl<'a, T> PlayerIterator<'a> for T
where
    T: Iterator<Item = &'a Player>,
{
    fn current_active_player(&'a mut self) -> &Player {
        self.find(|p| *p.active).unwrap()
    }

    fn some_player_not_ready(&'a mut self) -> bool {
        !self.all(|p| (*p.ready))
    }
}

pub trait PlayerIteratorMut<'a>: Iterator {
    fn set_next_active(&'a mut self, pos: usize);
}

impl<'a, T> PlayerIteratorMut<'a> for T
where
    T: Iterator<Item = Mut<'a, Player>>,
{
    fn set_next_active(&'a mut self, pos: usize) {
        for mut player in self.into_iter() {
            *player.active = false;
            if pos == *player.pos {
                *player.active = true;
            }
        }
    }
}

pub fn run_out_countdown(
    mut global: ResMut<Global>,
    mut countdown_q: Query<&mut Counter>,
    mut player_q: Query<&mut Player>,
    mut turn_q: Query<&mut Turn>,
    mut server: Server,
) {
    if let Ok(mut counter) = countdown_q.get_single_mut() {
        let is_over = counter.check_over();
        if is_over {
            info!("------------------ Game State: Run Out Countdown -----------------------");

            let cur_player = player_q.iter().current_active_player().clone();

            info!("Current Player: {}", cur_player.name());

            // FIXME: refactor!!!
            let mut turn = turn_q.get_single_mut().unwrap();

            turn.debug();

            let (leader_turn, Some(next_active_pos)) = turn.skip_turn() else {
                info!("Not found any next_active_pos -> End");
                return;
            };

            #[allow(unused_parens)]
            if (leader_turn) {
                // TODO:
                // Need to do some thing here,
                // Ex: auto play a card if player not play any cards
                info!("Leader turn not play card... Might counter some bug!");
            }

            info!("SEND ACTIVE: {}", next_active_pos);

            for (u_key, _) in global.users_map.iter() {
                server.send_message::<GameSystemChannel, UpdateTurn>(
                    u_key,
                    &UpdateTurn(next_active_pos),
                );
            }

            player_q.iter_mut().set_next_active(next_active_pos);
            global.cur_active_pos = next_active_pos;

            // for mut player in player_q.iter_mut() {
            //     *player.active = false;
            //     if next_active_pos == *player.pos {
            //         *player.active = true;
            //         global.cur_active_pos = next_active_pos;
            //     }
            // }

            info!("------------------ Game State: End Run Out Countdown -----------------------");
        }
    }
}

pub fn set_up_counter(mut commands: Commands) {
    commands.insert_resource(CounterConfig {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}
