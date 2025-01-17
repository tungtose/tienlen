use bevy::prelude::*;
use naia_bevy_client::{events::MessageEvents, Client};
use naia_bevy_demo_shared::{
    channels::{GameSystemChannel, PlayerActionChannel},
    messages::{EndMatch, ErrorCode, GameError, RequestStart, WaitForStart},
};
use std::time::Duration;

use crate::{assets::UiAssets, states::MainState};

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DrawStatus>()
            .add_event::<WaitForStartGame>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    handle_server_error_event,
                    handle_wait_event,
                    handle_end_match_event,
                ),
            )
            .add_systems(
                Update,
                (draw_status, delete_status, update_wait_for_status)
                    .run_if(in_state(MainState::Wait).or_else(in_state(MainState::Game))),
            );
    }
}

#[derive(Event, Default)]
pub struct WaitForStartGame;

pub enum WaitFor {
    EndMatch(usize),
    StartMatch(usize),
}

#[derive(Event)]
pub enum DrawStatus {
    Info(String),
    WaitFor(WaitFor),
    Error(String),
}

#[derive(Component)]
pub struct WaitForText(usize);

#[derive(Component)]
pub struct StatusContainer;

#[derive(Component)]
pub struct WaitForCounterConfig {
    timer: Timer,
}

#[derive(Component)]
pub struct CounterConfig {
    timer: Timer,
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        StatusContainer,
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                top: Val::Px(200.),
                width: Val::Percent(100.),
                height: Val::Px(48.),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

pub fn update_wait_for_status(
    mut commands: Commands,
    mut counter_q: Query<(Entity, &mut WaitForCounterConfig)>,
    mut text_q: Query<(&mut Text, &mut WaitForText)>,
    res: Res<UiAssets>,
    status_container_q: Query<Entity, With<StatusContainer>>,
    time: Res<Time>,
    mut client: Client,
) {
    for (entity, mut counter) in counter_q.iter_mut() {
        counter.timer.tick(time.delta());
        info!("TICK");

        if counter.timer.finished() {
            let status_container = status_container_q.get_single().unwrap();

            for (mut text, mut wait_for) in text_q.iter_mut() {
                let text_style = TextStyle {
                    font: res.font.clone(),
                    font_size: 16.0,
                    color: Color::YELLOW_GREEN,
                };

                wait_for.0 -= 1;

                let new_status = format!("Game start in {} seconds", wait_for.0);
                *text = Text::from_section(new_status, text_style.clone());

                if wait_for.0 == 0 {
                    commands.entity(entity).despawn();
                    commands.entity(status_container).despawn_descendants();

                    client
                        .send_message::<PlayerActionChannel, RequestStart>(&RequestStart::default());
                }
            }
        }
    }
}

pub fn delete_status(
    mut commands: Commands,
    mut counter_q: Query<(Entity, &mut CounterConfig)>,
    status_container_q: Query<Entity, With<StatusContainer>>,
    time: Res<Time>,
) {
    for (entity, mut counter) in counter_q.iter_mut() {
        counter.timer.tick(time.delta());

        if counter.timer.finished() {
            commands.entity(entity).despawn();

            let status_container = status_container_q.get_single().unwrap();

            commands.entity(status_container).despawn_descendants();
        }
    }
}

pub fn draw_status(
    mut commands: Commands,
    mut status_ev: EventReader<DrawStatus>,
    status_container_q: Query<Entity, With<StatusContainer>>,
    res: Res<UiAssets>,
) {
    let status_container = status_container_q.get_single().unwrap();

    for status in status_ev.read() {
        match status {
            DrawStatus::Error(msg) => {
                let status_text = commands
                    .spawn(TextBundle::from_section(
                        msg,
                        TextStyle {
                            font: res.font.clone(),
                            font_size: 16.0,
                            color: Color::ORANGE_RED,
                        },
                    ))
                    .id();

                commands.entity(status_container).add_child(status_text);
                commands.spawn(CounterConfig {
                    timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
                });
            }
            DrawStatus::Info(_) => todo!(),
            DrawStatus::WaitFor(wait_for) => match wait_for {
                WaitFor::StartMatch(time) => {
                    let msg = format!("Game start in {} seconds", time);
                    let status_text = commands
                        .spawn((
                            TextBundle::from_section(
                                msg,
                                TextStyle {
                                    font: res.font.clone(),
                                    font_size: 16.0,
                                    color: Color::YELLOW_GREEN,
                                },
                            ),
                            WaitForText(*time),
                        ))
                        .id();

                    commands.entity(status_container).add_child(status_text);

                    commands.spawn(WaitForCounterConfig {
                        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
                    });
                }
                WaitFor::EndMatch(time) => {
                    let msg = "Match ended".to_string();
                    let status_text = commands
                        .spawn((
                            TextBundle::from_section(
                                msg,
                                TextStyle {
                                    font: res.font.clone(),
                                    font_size: 16.0,
                                    color: Color::YELLOW_GREEN,
                                },
                            ),
                            WaitForText(*time),
                        ))
                        .id();

                    commands.entity(status_container).add_child(status_text);

                    commands.spawn(WaitForCounterConfig {
                        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
                    });
                }
            },
        }
    }
}

pub fn handle_wait_event(
    mut event_reader: EventReader<MessageEvents>,
    mut draw_status_ev: EventWriter<DrawStatus>,
    mut next_state: ResMut<NextState<MainState>>,
) {
    for events in event_reader.read() {
        for wait in events.read::<GameSystemChannel, WaitForStart>() {
            draw_status_ev.send(DrawStatus::WaitFor(WaitFor::StartMatch(wait.0)));
            next_state.set(MainState::Wait);
        }
    }
}

pub fn handle_end_match_event(
    mut event_reader: EventReader<MessageEvents>,
    mut next_state: ResMut<NextState<MainState>>,
    mut draw_status_ev: EventWriter<DrawStatus>,
) {
    for event in event_reader.read() {
        for end_match in event.read::<GameSystemChannel, EndMatch>() {
            draw_status_ev.send(DrawStatus::WaitFor(WaitFor::EndMatch(end_match.0)));
            next_state.set(MainState::Wait);
        }
    }
}

pub fn handle_server_error_event(
    mut event_reader: EventReader<MessageEvents>,
    mut draw_status_ev: EventWriter<DrawStatus>,
) {
    for events in event_reader.read() {
        for error_code in events.read::<GameSystemChannel, ErrorCode>() {
            let game_error = GameError::from(error_code);
            match game_error {
                GameError::InvalidCards => {
                    draw_status_ev.send(DrawStatus::Error("Your cards are week!".to_string()));
                }
                GameError::WrongCombination => {
                    draw_status_ev.send(DrawStatus::Error(
                        "Your cards are not the same combination".to_string(),
                    ));
                }
                GameError::CanNotSkipTurn => {
                    draw_status_ev.send(DrawStatus::Error(
                        "You can not skip turn, you can play any card now".to_string(),
                    ));
                }
                GameError::WrongTurn => {
                    draw_status_ev.send(DrawStatus::Error(
                        "Not your turn now! Game bug probably".to_string(),
                    ));
                }
                GameError::UnknownError => {
                    draw_status_ev.send(DrawStatus::Error("Unexpected error happend".to_string()));
                }
            }
        }
    }
}
