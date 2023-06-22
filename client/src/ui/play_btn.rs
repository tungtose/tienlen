use bevy::prelude::*;
use naia_bevy_client::{events::ClientTickEvent, Client};
use naia_bevy_demo_shared::{
    channels::PlayerCommandChannel,
    components::{player::Host, Player},
    messages::Game,
};

use crate::resources::Global;

use super::UiAssets;

const CONTAINER_HEIGHT: f32 = 50.;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
pub struct PlayContainer;

pub fn spawn_start_btn(
    mut commands: Commands,
    host_query: Query<&Host>,
    res: Res<UiAssets>,
    play_container_query: Query<Entity, With<PlayContainer>>,
    global: Res<Global>,
) {
    let play_container = play_container_query.get_single().unwrap();

    let mut is_host: bool = false;

    if let Some(player_entity) = global.player_entity {
        if let Ok(_host) = host_query.get(player_entity) {
            is_host = true; // true
        }
    }

    info!("IAM HOST!!! {}", is_host);

    if !is_host {
        return;
    }

    let start_btn = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(60.), Val::Px(40.)),
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
                        size: Size::new(Val::Px(60.), Val::Px(40.)),
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

    commands.entity(play_container).add_child(start_btn);
}

pub fn spawn_play_btn(mut commands: Commands, res: Res<UiAssets>) {
    let play_container = commands
        .spawn((
            PlayContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::bottom(Val::Px(70.)),
                    justify_content: JustifyContent::SpaceAround,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.), Val::Px(CONTAINER_HEIGHT)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let play_btn = commands
        .spawn(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(60.), Val::Px(40.)),
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
                        size: Size::new(Val::Px(60.), Val::Px(40.)),
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
                size: Size::new(Val::Px(60.), Val::Px(40.)),
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
                        size: Size::new(Val::Px(90.), Val::Px(40.)),
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

    let timmer = commands
        .spawn((
            SkipTurnTimerText,
            TextBundle::from_section(
                "00",
                TextStyle {
                    font: res.font.clone(),
                    font_size: 32.0,
                    color: Color::RED,
                },
            ),
        ))
        .id();

    commands
        .entity(play_container)
        .add_child(skip_btn)
        .add_child(timmer)
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

pub fn player_btn_click(
    mut interaction_query: Query<
        (
            &Interaction,
            (Option<&StartBtn>, Option<&PlayBtn>, Option<&SkipBtn>),
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut client: Client,
    mut tick_reader: EventReader<ClientTickEvent>,
    // mut ev_player: EventWriter<PlayerEvent>,
) {
    for (interaction, (start_btn, play_btn, skip_btn)) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if start_btn.is_some() {
                    info!("Clicked start!");
                    for ClientTickEvent(client_tick) in tick_reader.iter() {
                        let game = Game::new(true);
                        // Send command
                        client.send_tick_buffer_message::<PlayerCommandChannel, Game>(
                            client_tick,
                            &game.clone(),
                        );
                    }
                    // ev_player.send(PlayerEvent(PlayerEventKind::Play));
                }
                if play_btn.is_some() {
                    info!("Clicked play!");
                    // ev_player.send(PlayerEvent(PlayerEventKind::Play));
                }
                if skip_btn.is_some() {
                    info!("Clicked skip!");
                    // ev_player.send(PlayerEvent(PlayerEventKind::Skip));
                }
            }
            _ => {}
        }
    }
}
