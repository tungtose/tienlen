use std::time::Duration;

use bevy_ecs::system::{Commands, Query, Res, ResMut, Resource};
use bevy_time::{Time, Timer, TimerMode};
use naia_bevy_demo_shared::components::{timer::Counter, turn::Turn, Player};

use crate::resources::Global;

#[derive(Resource)]
pub struct CounterConfig {
    /// How often to spawn a new bomb? (repeating timer)
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

pub fn run_out_countdow(
    mut global: ResMut<Global>,
    mut countdown_q: Query<&mut Counter>,
    mut player_q: Query<&mut Player>,
    mut turn_q: Query<&mut Turn>,
) {
    if let Ok(mut counter) = countdown_q.get_single_mut() {
        let is_over = counter.check_over();
        if is_over {
            // FIXME: refactor!!!
            let mut turn = turn_q.get_single_mut().unwrap();
            let Some(next_active_pos) = turn.skip_turn() else {
                return;
            };

            for mut player in player_q.iter_mut() {
                *player.active = false;
                if next_active_pos == *player.pos {
                    *player.active = true;
                    global.cur_active_pos = next_active_pos;
                }
            }
        }
    }
}

pub fn set_up_counter(mut commands: Commands) {
    commands.insert_resource(CounterConfig {
        // create the repeating timer
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    })
}
