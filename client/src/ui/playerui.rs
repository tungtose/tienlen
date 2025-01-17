use std::time::Duration;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, text::Text2dBounds};

use crate::{components::LocalPlayer, resources::Global};

use super::{PlayerMessageEvent, UiAssets};

const AVATAR_SIZE: f32 = 55.;

#[derive(Component)]
pub struct ForeignPlayer;

#[derive(Component)]
pub struct PlayerTimerContainer;

#[derive(Component)]
pub struct PlayerMessageContainer;

#[derive(Component)]
pub struct AnimateText;

#[derive(Component)]
pub struct PlayerPos(pub i32);

#[derive(Component)]
pub struct Score;

#[derive(Component)]
pub struct Name;

#[derive(Component)]
pub struct CleanMessageCounter {
    timer: Timer,
    pos: i32,
}

pub fn update_score(
    mut text_q: Query<(&mut Text, &PlayerPos), With<Score>>,
    global: Res<Global>,
    res: Res<UiAssets>,
) {
    let text_style = TextStyle {
        font: res.font.clone(),
        font_size: 15.0,
        color: Color::WHITE,
    };

    // TODO: O(N^2) here, worst  case only 8 iterate but still bother me
    for (mut text, player_pos) in text_q.iter_mut() {
        if player_pos.0 == global.game.local_player.pos {
            let new_score = format!("Score: {}", global.game.local_player.score);
            *text = Text::from_section(new_score, text_style.clone());
        }

        if player_pos.0 == global.game.player_1.pos {
            let new_score = format!("Score: {}", global.game.player_1.score);
            *text = Text::from_section(new_score, text_style.clone());
        }

        if player_pos.0 == global.game.player_2.pos {
            let new_score = format!("Score: {}", global.game.player_2.score);
            *text = Text::from_section(new_score, text_style.clone());
        }

        if player_pos.0 == global.game.player_3.pos {
            let new_score = format!("Score: {}", global.game.player_3.score);
            *text = Text::from_section(new_score, text_style.clone());
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn update_timer(
    mut text_q: Query<
        (&mut Visibility, &PlayerPos, &mut Text),
        (With<PlayerTimerContainer>, With<Text>),
    >,
    global: Res<Global>,
    res: Res<UiAssets>,
) {
    let timer_text = TextStyle {
        font: res.font.clone(),
        font_size: 20.0,
        color: Color::YELLOW_GREEN,
    };

    for (mut vis, player_pos, mut text) in text_q.iter_mut() {
        let time = global.game.timer.parse::<f32>().unwrap();

        if (0.0..5.0).contains(&time) {
            if player_pos.0 == global.game.active_player_pos {
                let time = &global.game.timer;
                *text = Text::from_section(time, timer_text.clone());
                *vis = Visibility::Visible;
            }
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn animatetext_update(
    mut text_q: Query<(&mut Visibility, &PlayerPos, &mut Text), (With<AnimateText>, With<Text>)>,
    global: Res<Global>,
    res: Res<UiAssets>,
) {
    let playing_text = TextStyle {
        font: res.font.clone(),
        font_size: 16.0,
        color: Color::ORANGE_RED,
    };

    let idle_text = TextStyle {
        font: res.font.clone(),
        font_size: 16.0,
        color: Color::WHITE,
    };
    for (mut vis, player_pos, mut text) in text_q.iter_mut() {
        if player_pos.0 == global.game.active_player_pos {
            let name = text.sections.first().unwrap().value.clone();
            *text = Text::from_section(name, playing_text.clone());
            match *vis {
                Visibility::Hidden => *vis = Visibility::Visible,
                Visibility::Visible => *vis = Visibility::Hidden,
                Visibility::Inherited => *vis = Visibility::Visible,
            }

            continue;
        }

        let name = text.sections.first().unwrap().value.clone();
        *text = Text::from_section(name, idle_text.clone());

        *vis = Visibility::Visible;
    }
}

#[derive(Component)]
pub struct ForeignPlayer1;
#[derive(Component)]
pub struct ForeignPlayer2;
#[derive(Component)]
pub struct ForeignPlayer3;

pub fn draw_player_ui(
    mut commands: Commands,
    mut global: ResMut<Global>,
    res: Res<UiAssets>,
    mut material: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let local_player = &global.game.local_player;

    if local_player.is_join && !local_player.is_drawed {
        let local_player_ui = create_player_ui(
            &mut commands,
            &local_player.draw_pos,
            &res,
            local_player.pos,
            &local_player.score.to_string(),
            &local_player.name,
        );

        commands.entity(local_player_ui).insert(LocalPlayer);

        global.game.local_player.is_drawed = true;
    }

    if global.game.player_1.is_join && !global.game.player_1.is_drawed {
        let p1 = &global.game.player_1;
        let p1_ui = create_player_ui(
            &mut commands,
            &p1.draw_pos,
            &res,
            p1.pos,
            &p1.score.to_string(),
            &p1.name,
        );

        commands.entity(p1_ui).insert(ForeignPlayer1);

        global.game.player_1.is_drawed = true;
    }

    let p2 = &global.game.player_2;
    if p2.is_join && !p2.is_drawed {
        let p2_ui = create_player_ui(
            &mut commands,
            &p2.draw_pos,
            &res,
            p2.pos,
            &p2.score.to_string(),
            &p2.name,
        );
        commands.entity(p2_ui).insert(ForeignPlayer2);

        global.game.player_2.is_drawed = true;
    }

    let p3 = &global.game.player_3;
    if p3.is_join && !p3.is_drawed {
        let p3_ui = create_player_ui(
            &mut commands,
            &p3.draw_pos,
            &res,
            p3.pos,
            &p3.score.to_string(),
            &p3.name,
        );

        commands.entity(p3_ui).insert(ForeignPlayer3);

        global.game.player_3.is_drawed = true;
    }
}

pub fn clean_player_message(
    mut commands: Commands,
    time: Res<Time>,
    res: Res<UiAssets>,
    mut counter_q: Query<(Entity, &mut CleanMessageCounter)>,
    mut ui_q: Query<(&mut Text, &PlayerPos), With<PlayerMessageContainer>>,
) {
    let text_style = TextStyle {
        font: res.font.clone(),
        font_size: 16.0,
        color: Color::rgb(0.9, 0.9, 0.9),
    };
    for (entity, mut counter) in counter_q.iter_mut() {
        counter.timer.tick(time.delta());

        if counter.timer.finished() {
            commands.entity(entity).despawn();
            for (mut text, pos) in ui_q.iter_mut() {
                if pos.0 == counter.pos {
                    *text = Text::from_section("".to_string(), text_style.clone());
                }
            }
        }
    }
}

pub fn update_player_message(
    mut commands: Commands,
    mut message_ev: EventReader<PlayerMessageEvent>,
    mut ui_q: Query<(&mut Text, &PlayerPos), With<PlayerMessageContainer>>,
    res: Res<UiAssets>,
) {
    let text_style = TextStyle {
        font: res.font.clone(),
        font_size: 16.0,
        color: Color::DARK_GREEN,
    };

    for message in message_ev.iter() {
        for (mut text, pos) in ui_q.iter_mut() {
            if pos.0 == message.0 as i32 {
                *text = Text::from_section(message.1.clone(), text_style.clone());

                commands.spawn(CleanMessageCounter {
                    timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
                    pos: pos.0,
                });
            }
        }
    }
}

pub fn create_player_ui(
    commands: &mut Commands,
    draw_pos: &Vec2,
    res: &Res<UiAssets>,
    player_pos: i32,
    player_score: &str,
    player_name: &str,
) -> Entity {
    let text_style = TextStyle {
        font: res.font.clone(),
        font_size: 15.0,
        color: Color::WHITE,
    };

    let avatar_handle = res.avatars.get(&player_pos).unwrap().clone();
    let score = format!("Score: {}", player_score);

    let avatar = SpriteBundle {
        texture: avatar_handle,
        transform: Transform::from_xyz(draw_pos.x, draw_pos.y, 5.),
        sprite: Sprite {
            custom_size: Some(Vec2::new(AVATAR_SIZE, AVATAR_SIZE)),
            ..default()
        },
        ..default()
    };

    commands
        .spawn((avatar, PlayerPos(player_pos)))
        .with_children(|builder| {
            builder.spawn((
                Name,
                AnimateText,
                PlayerPos(player_pos),
                Text2dBundle {
                    text: Text::from_section(player_name, text_style.clone())
                        .with_alignment(TextAlignment::Left),
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(100., 30.),
                    },
                    transform: Transform::from_translation(Vec3::from_array([
                        draw_pos.x,
                        draw_pos.y + 35.,
                        1.,
                    ])),
                    ..default()
                },
            ));
        })
        .with_children(|builder| {
            builder.spawn((
                Score,
                PlayerPos(player_pos),
                Text2dBundle {
                    text: Text::from_section(score, text_style.clone())
                        .with_alignment(TextAlignment::Center),
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(100., 30.),
                    },
                    transform: Transform::from_translation(Vec3::from_array([
                        draw_pos.x,
                        draw_pos.y - 35.,
                        1.,
                    ])),
                    ..default()
                },
            ));
        })
        .with_children(|builder| {
            builder.spawn((
                PlayerMessageContainer,
                PlayerPos(player_pos),
                Text2dBundle {
                    text: Text::from_section("".to_string(), text_style.clone())
                        .with_alignment(TextAlignment::Center),
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(100., 30.),
                    },
                    transform: Transform::from_translation(Vec3::from_array([
                        draw_pos.x + 20.,
                        draw_pos.y + 45.,
                        1.,
                    ])),
                    ..default()
                },
            ));
        })
        .with_children(|builder| {
            builder.spawn((
                PlayerTimerContainer,
                PlayerPos(player_pos),
                Text2dBundle {
                    text: Text::from_section("0".to_string(), text_style)
                        .with_alignment(TextAlignment::Center),
                    text_2d_bounds: Text2dBounds {
                        size: Vec2::new(100., 30.),
                    },
                    visibility: Visibility::Hidden,
                    transform: Transform::from_translation(Vec3::from_array([
                        draw_pos.x + 40.,
                        draw_pos.y,
                        1.,
                    ])),
                    ..default()
                },
            ));
        })
        .id()
}
