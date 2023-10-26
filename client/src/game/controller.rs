use bevy::prelude::*;
use naia_bevy_client::{events::MessageEvents, Client};
use naia_bevy_demo_shared::{
    channels::{EntityAssignmentChannel, GameSystemChannel, PlayerActionChannel},
    components::Host,
    messages::{
        AcceptPlayCard, AcceptStartGame, EndMatch, EntityAssignment, NewMatch, StartGame,
        UpdateTurn,
    },
};

use crate::{resources::Global, states::MainState, ui::UiAssets};

use super::{
    cards::{CStatus, Card, Ordinal},
    player_ui::{Bottom, PlayerPos},
};

pub struct ControllerPlugin;

impl Plugin for ControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayEvent>()
            .add_event::<SkipTurnEvent>()
            .add_systems(
                Update,
                (
                    spawn_start_btn,
                    handle_start_game_event,
                    handle_end_match_event,
                ),
            )
            .add_systems(OnEnter(MainState::Lobby), spawn_play_controller)
            .add_systems(OnEnter(MainState::Wait), hide_start_btn)
            .add_systems(Update, player_btn_click.run_if(in_state(MainState::Lobby)))
            .add_systems(
                Update,
                (player_btn_click, update_play_controller, handle_skip_event)
                    .run_if(in_state(MainState::Game).or_else(in_state(MainState::Wait))),
            );
    }
}

const CONTAINER_HEIGHT: f32 = 50.;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Event, Default)]
pub struct SkipTurnEvent;

#[derive(Event, Clone, Default)]
pub struct PlayEvent(pub Vec<Entity>);

#[derive(Component)]
pub struct PlayContainer;

#[derive(Component)]
pub struct StartContainer;

pub fn hide_start_btn(mut vis_q: Query<&mut Visibility, With<StartContainer>>) {
    for mut vis in vis_q.iter_mut() {
        *vis = Visibility::Hidden;
    }
}

pub fn spawn_start_btn(
    mut commands: Commands,
    mut event_reader: EventReader<MessageEvents>,
    host_query: Query<&Host>,
    res: Res<UiAssets>,
    global: Res<Global>,
) {
    for events in event_reader.iter() {
        for _message in events.read::<EntityAssignmentChannel, EntityAssignment>() {
            if let Some(player_entity) = global.player_entity {
                // This player is not a host
                if host_query.get(player_entity).is_err() {
                    return;
                }
            }

            let container = commands
                .spawn((
                    StartContainer,
                    NodeBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            top: Val::Px(150.),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Percent(100.),
                            height: Val::Px(100.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ))
                .id();

            let start_btn = commands
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(60.),
                        height: Val::Px(40.),
                        margin: UiRect::all(Val::Px(4.)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(ButtonBundle {
                            style: Style {
                                width: Val::Px(60.),
                                height: Val::Px(40.),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                ..default()
                            },

                            background_color: NORMAL_BUTTON.into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Start",
                                TextStyle {
                                    font: res.font.clone(),
                                    font_size: 16.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            ));
                        })
                        .insert(StartBtn);
                })
                .id();

            commands.entity(container).add_child(start_btn);
        }
    }
}

pub fn handle_skip_event(
    mut vis_q: Query<&mut Visibility, With<PlayContainer>>,
    mut event_reader: EventReader<MessageEvents>,
    player_q: Query<&PlayerPos, With<Bottom>>,
) {
    for event in event_reader.iter() {
        for message in event.read::<GameSystemChannel, UpdateTurn>() {
            let mut vis = vis_q.get_single_mut().unwrap();
            for player_pos in player_q.iter() {
                if player_pos.0 == message.0 as i32 {
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn handle_start_game_event(
    mut vis_q: Query<&mut Visibility, With<PlayContainer>>,
    mut event_reader: EventReader<MessageEvents>,
    player_q: Query<&PlayerPos, With<Bottom>>,
) {
    for event in event_reader.iter() {
        for message in event.read::<GameSystemChannel, AcceptStartGame>() {
            let mut vis = vis_q.get_single_mut().unwrap();
            for player_pos in player_q.iter() {
                if player_pos.0 == message.active_player as i32 {
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn update_play_controller(
    mut vis_q: Query<&mut Visibility, With<PlayContainer>>,
    mut event_reader: EventReader<MessageEvents>,
    player_q: Query<&PlayerPos, With<Bottom>>,
) {
    for event in event_reader.iter() {
        for message in event.read::<GameSystemChannel, AcceptPlayCard>() {
            let mut vis = vis_q.get_single_mut().unwrap();
            for player_pos in player_q.iter() {
                if player_pos.0 == message.next_player as i32 {
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
                }
            }
        }

        for message in event.read::<GameSystemChannel, NewMatch>() {
            let mut vis = vis_q.get_single_mut().unwrap();
            for player_pos in player_q.iter() {
                if player_pos.0 == message.active_player as i32 {
                    *vis = Visibility::Visible;
                } else {
                    *vis = Visibility::Hidden;
                }
            }
        }
    }
}

pub fn spawn_play_controller(mut commands: Commands, res: Res<UiAssets>) {
    let show_player_control = Visibility::Hidden;

    let play_container = commands
        .spawn((
            PlayContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(230.),
                    right: Val::Px(230.),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    width: Val::Px(100.),
                    height: Val::Px(CONTAINER_HEIGHT),
                    ..Default::default()
                },
                visibility: show_player_control,
                ..Default::default()
            },
        ))
        .id();

    let play_btn = commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(30.),
                margin: UiRect::all(Val::Px(4.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(50.),
                        height: Val::Px(30.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GREEN),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font: res.font.clone(),
                            font_size: 12.0,
                            color: Color::ANTIQUE_WHITE,
                        },
                    ));
                })
                .insert(PlayBtn);
        })
        .id();

    let skip_btn = commands
        .spawn(ButtonBundle {
            style: Style {
                width: Val::Px(50.),
                height: Val::Px(30.),
                margin: UiRect::all(Val::Px(4.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn(ButtonBundle {
                    style: Style {
                        width: Val::Px(50.),
                        height: Val::Px(30.),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::ORANGE_RED),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Skip",
                        TextStyle {
                            font: res.font.clone(),
                            font_size: 12.0,
                            color: Color::ANTIQUE_WHITE,
                        },
                    ));
                })
                .insert(SkipBtn);
        })
        .id();

    commands
        .entity(play_container)
        .add_child(skip_btn)
        .add_child(play_btn);
}

#[derive(Component)]
pub struct SkipTurnTimerText;

#[derive(Component)]
pub struct StartBtn;

#[derive(Component)]
pub struct PlayBtn;

#[derive(Component)]
pub struct SkipBtn;

#[allow(clippy::type_complexity)]
pub fn player_btn_click(
    mut interaction_query: Query<
        (
            &Interaction,
            (Option<&StartBtn>, Option<&PlayBtn>, Option<&SkipBtn>),
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut client: Client,
    mut play_event_writer: EventWriter<PlayEvent>,
    mut skip_ev: EventWriter<SkipTurnEvent>,
    card_q: Query<(Entity, &CStatus, &Ordinal), With<Card>>,
) {
    for (interaction, (start_btn, play_btn, skip_btn)) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if start_btn.is_some() {
                // info!("Clicked start!");
                client.send_message::<PlayerActionChannel, StartGame>(&StartGame::default());
            }
            if play_btn.is_some() {
                // info!("Clicked play!");
                let mut cards = vec![];

                for (entity, status, ordinal) in card_q.iter() {
                    if let CStatus::Active = *status {
                        cards.push((entity, ordinal.get()));
                    }
                }

                cards.sort_by_key(|c| c.1);
                play_event_writer.send(PlayEvent(cards.iter().map(|c| c.0).collect()));
            }
            if skip_btn.is_some() {
                // info!("Clicked skip!");
                skip_ev.send_default()
            }
        }
    }
}

pub fn handle_end_match_event(
    mut event_reader: EventReader<MessageEvents>,
    mut vis_q: Query<&mut Visibility, With<PlayContainer>>,
) {
    for event in event_reader.iter() {
        for end_match in event.read::<GameSystemChannel, EndMatch>() {
            for mut vis in vis_q.iter_mut() {
                *vis = Visibility::Hidden;
            }
        }
    }
}
