use std::time::Duration;

use bevy::prelude::*;

use crate::{
    deck::ActiveCard,
    player::{ActivePlayer, Player, PlayerHand, PlayerPosition, TurnUpdateEvent},
    states::{GameState, MainState, TurnSet},
};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            (TurnSet::Logic, TurnSet::Animation, TurnSet::Tick)
                .chain()
                .in_set(OnUpdate(GameState::TurnUpdate)),
        )
        .add_system(setup_turn_skip_timer.in_schedule(OnEnter(MainState::Game)))
        .add_system(turn_skip.run_if(in_state(MainState::Game)))
        .add_system(game_start.in_schedule(OnEnter(MainState::Game)))
        .add_system(game_end.in_schedule(OnExit(MainState::Game)))
        .add_system(turn_update_start.run_if(on_event::<TurnUpdateEvent>()))
        .add_system(turn_update.in_schedule(OnEnter(GameState::TurnUpdate)));
    }
}

fn game_start(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::PlayerInput);
}

fn game_end(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::None);
}

fn turn_update_start(
    mut next_state: ResMut<NextState<GameState>>,
    // mut ev_tick: EventWriter<TickEvent>,
) {
    next_state.set(GameState::TurnUpdate);
}

fn turn_update(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    mut active_player_query: Query<(Entity, &PlayerPosition, &mut PlayerHand), With<ActivePlayer>>,
    mut active_cards: Query<&mut ActiveCard>,
    mut player_query: Query<(Entity, &PlayerPosition), With<Player>>,
    mut config: ResMut<TurnSkipConfig>,
) {
    let (cur_active_player, cur_player_pos, mut cur_player_hand) =
        active_player_query.get_single_mut().unwrap();

    // Calculate next turn
    let next_turn = {
        let pos = cur_player_pos.0 % 4 + 1;
        let res = if pos == 4 { 0 } else { pos };
        res
    };

    info!(
        "RUNNING TURN UPDATE!, {:?}, {:?}",
        cur_player_pos.0, next_turn
    );

    // Clear active cards
    cur_player_hand.active_cards.clear();

    for mut card in active_cards.iter_mut() {
        card.0 = false;
    }

    // Transfer next Active Player
    commands.entity(cur_active_player).remove::<ActivePlayer>();

    for (entity, player_pos) in player_query.iter_mut() {
        if player_pos.0 == next_turn {
            commands.entity(entity).insert(ActivePlayer);
            break;
        }
    }

    // Reset timer
    config.timer.reset();
    next_state.set(GameState::PlayerInput);
}

#[derive(Resource)]
pub struct TurnSkipConfig {
    pub timer: Timer,
}

fn turn_skip(
    time: Res<Time>,
    mut config: ResMut<TurnSkipConfig>,
    mut ev_turn_update: EventWriter<TurnUpdateEvent>,
) {
    config.timer.tick(time.delta());
    if config.timer.finished() {
        ev_turn_update.send(TurnUpdateEvent);
    }
}

fn setup_turn_skip_timer(mut commands: Commands) {
    commands.insert_resource(TurnSkipConfig {
        timer: Timer::new(Duration::from_secs(15), TimerMode::Repeating),
    })
}

// fn turn_update_end(mut next_state: ResMut<NextState<GameState>>) {
//     next_state.set(GameState::PlayerInput);
// }
//
// fn turn_update_cancel(mut next_state: ResMut<NextState<GameState>>) {
//     next_state.set(GameState::PlayerInput);
// }
