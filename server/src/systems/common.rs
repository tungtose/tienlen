use std::time::Duration;

use bevy_ecs::{
    system::{Commands, Query, Res, ResMut, Resource},
    world::Mut,
};
use bevy_log::info;
use bevy_time::{Time, Timer, TimerMode};
use naia_bevy_demo_shared::{
    channels::GameSystemChannel,
    components::{hand::Hand, timer::Counter, turn::Turn, Player, Table},
    messages::{AcceptPlayCard, UpdateTurn},
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
        for mut counter in countdown_q.iter_mut() {
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
    fn update_active_player_cards(&'a mut self, cards: &str);
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

    fn update_active_player_cards(&'a mut self, cards: &str) {
        for mut player in self.into_iter() {
            if *player.active {
                *player.cards = cards.to_string();
            }
        }
    }
}

pub fn run_out_countdown(
    mut global: ResMut<Global>,
    mut countdown_q: Query<&mut Counter>,
    mut player_q: Query<&mut Player>,
    mut turn_q: Query<&mut Turn>,
    mut table_q: Query<&mut Table>,
    mut server: Server,
) {
    if let Ok(mut counter) = countdown_q.get_single_mut() {
        if counter.check_over() {
            info!("------------------ Game State: Run Out Countdown -----------------------");

            let cur_player = player_q.iter().current_active_player().clone();
            let mut turn = turn_q.get_single_mut().unwrap();

            if global.leader_turn {
                *counter.counter = 3.;

                let next_active_pos = turn.next_turn().unwrap();

                let mut hand = Hand::from(cur_player.cards());

                let card_played = hand.remove_smallest_card().to_str();

                player_q
                    .iter_mut()
                    .update_active_player_cards(&hand.to_string());

                let mut table = table_q.get_single_mut().unwrap();
                *table.cards = card_played.clone();

                global.table.push_back(Hand::from(card_played.clone()));
                // *counter.counter = 5.;

                let data = AcceptPlayCard {
                    cur_player: *cur_player.pos,
                    cards: card_played,
                    next_player: next_active_pos,
                    run_out_card: false,
                };

                for (user_key, _) in global.users_map.iter() {
                    server.send_message::<GameSystemChannel, AcceptPlayCard>(user_key, &data);
                }

                global.leader_turn = false;

                player_q.iter_mut().set_next_active(next_active_pos);
                global.cur_active_pos = next_active_pos;

                counter.recount();

                return;
            }

            let (next_active_leader_turn, Some(next_active_pos)) = turn.skip_turn() else {
                info!("Not found any next_active_pos -> End");
                return;
            };

            if next_active_leader_turn {
                global.leader_turn = true;
            }

            for (u_key, _) in global.users_map.iter() {
                server.send_message::<GameSystemChannel, UpdateTurn>(
                    u_key,
                    &UpdateTurn(next_active_pos),
                );
            }

            player_q.iter_mut().set_next_active(next_active_pos);
            global.cur_active_pos = next_active_pos;

            counter.recount();

            info!("------------------ Game State: End Run Out Countdown -----------------------");
        }
    }
}

pub fn set_up_counter(mut commands: Commands) {
    commands.insert_resource(CounterConfig {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}
