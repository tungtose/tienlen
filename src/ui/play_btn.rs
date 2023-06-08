use crate::{
    manager::TurnSkipConfig,
    player::{PlayerEvent, PlayerEventKind},
};
use bevy::prelude::*;

use super::UiAssets;

const CONTAINER_HEIGHT: f32 = 50.;
const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Component)]
pub struct PlayContainer;

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

pub fn update_turn_timer(
    mut timer_text_query: Query<&mut Text, With<SkipTurnTimerText>>,
    res: ResMut<TurnSkipConfig>,
) {
    for mut text in timer_text_query.iter_mut() {
        let cur_time = res.timer.remaining_secs().floor() + 1.;
        text.sections[0].value = cur_time.to_string();
    }
}

#[derive(Component)]
pub struct SkipTurnTimerText;

#[derive(Component)]
pub struct PlayBtn;

#[derive(Component)]
pub struct SkipBtn;

pub fn player_btn_click(
    mut interaction_query: Query<
        (&Interaction, (Option<&PlayBtn>, Option<&SkipBtn>)),
        (Changed<Interaction>, With<Button>),
    >,
    mut ev_player: EventWriter<PlayerEvent>,
) {
    for (interaction, (play_btn, skip_btn)) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                if play_btn.is_some() {
                    ev_player.send(PlayerEvent(PlayerEventKind::Play));
                }
                if skip_btn.is_some() {
                    ev_player.send(PlayerEvent(PlayerEventKind::Skip));
                }
            }
            _ => {}
        }
    }
}
