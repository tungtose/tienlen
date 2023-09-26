use bevy::prelude::*;
use naia_bevy_client::Client;
use naia_bevy_demo_shared::{
    channels::PlayerActionChannel,
    components::{player::Host, Player},
    messages::StartGame,
};

use crate::{
    components::LocalPlayer,
    game::{PlayerEvent, SkipTurnEvent},
    resources::Global,
};

use super::UiAssets;

const CONTAINER_HEIGHT: f32 = 50.;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

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
    host_query: Query<&Host>,
    res: Res<UiAssets>,
    global: Res<Global>,
) {
    let mut is_host: bool = false;

    if let Some(player_entity) = global.player_entity {
        if let Ok(_host) = host_query.get(player_entity) {
            is_host = true; // true
        }
    }

    if !is_host {
        return;
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

pub fn spawn_play_btn(
    mut commands: Commands,
    res: Res<UiAssets>,
    player_container_q: Query<Entity, With<PlayContainer>>,
    player_q: Query<&Player, With<LocalPlayer>>,
) {
    let Ok(player) = player_q.get_single() else {
        return;
    };

    for player_container_entity in player_container_q.iter() {
        commands.entity(player_container_entity).despawn_recursive();
    }

    let show_player_control = if *player.active {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };

    let play_container = commands
        .spawn((
            PlayContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(40.),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
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
                        "Play",
                        TextStyle {
                            font: res.font.clone(),
                            font_size: 16.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                })
                .insert(PlayBtn);
        })
        .id();

    let skip_btn = commands
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
                        width: Val::Px(90.),
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
                        "Skip Turn",
                        TextStyle {
                            font: res.font.clone(),
                            font_size: 16.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
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
    mut player_ev: EventWriter<PlayerEvent>,
    mut skip_ev: EventWriter<SkipTurnEvent>,
) {
    for (interaction, (start_btn, play_btn, skip_btn)) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if start_btn.is_some() {
                info!("Clicked start!");
                client.send_message::<PlayerActionChannel, StartGame>(&StartGame::default());
            }
            if play_btn.is_some() {
                info!("Clicked play!");
                player_ev.send(PlayerEvent)
            }
            if skip_btn.is_some() {
                info!("Clicked skip!");
                skip_ev.send(SkipTurnEvent)
            }
        }
    }
}
