use bevy::prelude::*;
use naia_bevy_client::events::MessageEvents;
use naia_bevy_demo_shared::{
    channels::GameSystemChannel,
    messages::{ErrorCode, GameError},
};
use std::time::Duration;

use crate::{states::MainState, ui::UiAssets};

pub struct StatusPlugin;

impl Plugin for StatusPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<DrawStatus>()
            .add_systems(OnEnter(MainState::Game), setup)
            .add_systems(Update, handle_server_error_event)
            .add_systems(
                Update,
                (draw_status, delete_status).run_if(in_state(MainState::Game)),
            );
    }
}

#[derive(Event)]
pub enum DrawStatus {
    Info(String),
    Error(String),
}

#[derive(Component)]
pub struct StatusContainer;

#[derive(Component)]
pub struct CounterConfig {
    timer: Timer,
}

pub fn setup(mut commands: Commands, res: Res<UiAssets>) {
    commands.spawn((
        StatusContainer,
        NodeBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.),
                height: Val::Px(48.),
                ..Default::default()
            },
            ..Default::default()
        },
    ));
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

    for status in status_ev.iter() {
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
        }
    }
}

pub fn handle_server_error_event(
    mut event_reader: EventReader<MessageEvents>,
    mut draw_status_ev: EventWriter<DrawStatus>,
) {
    for events in event_reader.iter() {
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
