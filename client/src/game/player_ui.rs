use std::time::Duration;

use bevy::{prelude::*, text::Text2dBounds, time::common_conditions::on_fixed_timer};

use crate::{resources::Global, states::MainState, ui::UiAssets};

pub struct PlayerUiPlugin;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMessageEvent>()
            .add_systems(Update, draw_player_ui.run_if(in_state(MainState::Lobby)))
            .add_systems(Update, update_score.run_if(in_state(MainState::Game)))
            .add_systems(
                Update,
                clean_player_message.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                update_player_message.run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                animatetext_update.run_if(on_fixed_timer(Duration::from_millis(500))),
            )
            .add_systems(Update, update_timer.run_if(in_state(MainState::Game)));
    }
}

const AVATAR_SIZE: f32 = 55.;

#[derive(Default, Event)]
pub struct PlayerMessageEvent(pub usize, pub String);

#[derive(Component)]
pub struct BackCard;

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
                info!("Update TIMMER");
                let time = &global.game.timer;
                *text = Text::from_section(time, timer_text.clone());
                *vis = Visibility::Visible;
            }
        } else {
            *vis = Visibility::Hidden;
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

pub fn draw_player_ui(mut commands: Commands, mut global: ResMut<Global>, res: Res<UiAssets>) {
    let local_player = &global.game.local_player;

    if local_player.is_join && !local_player.is_drawed {
        create_player_ui(
            &mut commands,
            &local_player.draw_pos,
            &res,
            local_player.pos,
            &local_player.score.to_string(),
            &local_player.name,
        );

        global.game.local_player.is_drawed = true;
    }

    if global.game.player_1.is_join && !global.game.player_1.is_drawed {
        let p1 = &global.game.player_1;
        create_player_ui(
            &mut commands,
            &p1.draw_pos,
            &res,
            p1.pos,
            &p1.score.to_string(),
            &p1.name,
        );

        global.game.player_1.is_drawed = true;
    }

    let p2 = &global.game.player_2;
    if p2.is_join && !p2.is_drawed {
        create_player_ui(
            &mut commands,
            &p2.draw_pos,
            &res,
            p2.pos,
            &p2.score.to_string(),
            &p2.name,
        );

        global.game.player_2.is_drawed = true;
    }

    let p3 = &global.game.player_3;
    if p3.is_join && !p3.is_drawed {
        create_player_ui(
            &mut commands,
            &p3.draw_pos,
            &res,
            p3.pos,
            &p3.score.to_string(),
            &p3.name,
        );

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

pub fn create_player_ui(
    commands: &mut Commands,
    draw_pos: &Vec2,
    res: &Res<UiAssets>,
    player_pos: i32,
    player_score: &str,
    player_name: &str,
) {
    let text_style = TextStyle {
        font: res.font.clone(),
        font_size: 15.0,
        color: Color::WHITE,
    };

    let avatar_handle = res.avatars.get(&player_pos).unwrap().clone();
    let score = format!("Score: {}", player_score);

    let avatar = SpriteBundle {
        transform: Transform::from_xyz(draw_pos.x, draw_pos.y, 5.),
        texture: avatar_handle,
        sprite: Sprite {
            custom_size: Some(Vec2::new(AVATAR_SIZE, AVATAR_SIZE)),
            ..default()
        },
        ..default()
    };

    commands.spawn((
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
                15.,
            ])),
            ..default()
        },
    ));

    commands.spawn((
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
                15.,
            ])),
            ..default()
        },
    ));

    commands.spawn((
        PlayerTimerContainer,
        PlayerPos(player_pos),
        Text2dBundle {
            text: Text::from_section("0".to_string(), text_style.clone())
                .with_alignment(TextAlignment::Center),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(100., 30.),
            },
            visibility: Visibility::Hidden,
            transform: Transform::from_translation(Vec3::from_array([
                draw_pos.x + 40.,
                draw_pos.y,
                15.,
            ])),
            ..default()
        },
    ));

    commands.spawn((
        PlayerMessageContainer,
        PlayerPos(player_pos),
        Text2dBundle {
            text: Text::from_section("".to_string(), text_style)
                .with_alignment(TextAlignment::Center),
            text_2d_bounds: Text2dBounds {
                size: Vec2::new(100., 30.),
            },
            transform: Transform::from_translation(Vec3::from_array([
                draw_pos.x + 20.,
                draw_pos.y + 45.,
                15.,
            ])),
            ..default()
        },
    ));

    let back_card_handle = res.back_card.clone();
    let back_card_margin = 60.;

    commands.spawn((
        BackCard,
        PlayerPos(player_pos),
        SpriteBundle {
            texture: back_card_handle,
            sprite: Sprite {
                custom_size: Some(Vec2::new(30., 45.)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::from_array([
                draw_pos.x + back_card_margin,
                draw_pos.y,
                15.,
            ])),
            ..default()
        },
    ));

    commands.spawn((avatar, PlayerPos(player_pos)));
}
