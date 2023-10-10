use std::time::Duration;

use bevy::{prelude::*, text::Text2dBounds};
use bevy_prototype_lyon::prelude::*;

use crate::{components::LocalPlayer, resources::Global};

use super::{PlayerMessageEvent, UiAssets};

const TIME_OUT: f32 = 20.;
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
pub struct Cooldown;

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

#[derive(Component)]
pub struct CircleSetting {
    angle: f32,
    center: Vec2,
    radii: Vec2,
    rotate: f32,
    pos: Vec2,
}

impl CircleSetting {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            angle: 0.0,
            center: pos,
            radii: Vec2::new(30., 30.),
            rotate: 0.,
        }
    }

    pub fn set_zero_degree(&mut self) {
        self.angle = f32::to_radians(0.);
    }

    pub fn validate_arc(&mut self) {
        if self.angle < f32::to_radians(-360.) || self.angle > f32::to_radians(360.) {
            self.set_zero_degree();
        }
    }

    pub fn angle_degree(&self) -> f32 {
        f32::to_degrees(self.angle)
    }

    pub fn update_angle(&mut self, delta: f32) {
        self.angle = f32::to_radians(self.angle_degree() - (360. / (TIME_OUT + 1.)) * delta);
        self.validate_arc();
    }

    pub fn create_circle_path(&self) -> Path {
        let mut path_builder = PathBuilder::new();

        path_builder.move_to(self.pos);
        path_builder.line_to(Vec2::new(self.pos.x, self.radii.y + self.pos.y));
        path_builder.arc(self.center, self.radii, self.angle, self.rotate);

        path_builder.close();
        path_builder.build()
    }

    pub fn update_circle_path(&mut self, delta: f32) -> Path {
        self.update_angle(delta);
        self.create_circle_path()
    }
}

#[allow(clippy::type_complexity)]
pub fn circle_cooldown_update(
    mut player_ui_q: Query<(&mut CircleSetting, &mut Path, &PlayerPos), With<Cooldown>>,
    global: Res<Global>,
    time: Res<Time>,
) {
    // for (mut cir_setting, mut path, player_pos) in player_ui_q.iter_mut() {
    //     if global.game.active_player_pos == player_pos.0 {
    //         let delta = time.delta_seconds();
    //         let new_path = cir_setting.update_circle_path(delta);
    //         *path = new_path;
    //         continue;
    //     }
    //     cir_setting.set_zero_degree();
    //     let new_path = create_circle_path(&cir_setting);
    //     *path = new_path;
    // }
}

fn create_circle_path(cir_setting: &CircleSetting) -> Path {
    let mut path_builder = PathBuilder::new();

    path_builder.move_to(cir_setting.pos);
    path_builder.line_to(Vec2::new(
        cir_setting.pos.x,
        cir_setting.radii.y + cir_setting.pos.y,
    ));
    path_builder.arc(
        cir_setting.center,
        cir_setting.radii,
        cir_setting.angle,
        cir_setting.rotate,
    );

    path_builder.close();
    path_builder.build()
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

pub fn draw_player_ui(mut commands: Commands, mut global: ResMut<Global>, res: Res<UiAssets>) {
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
    let cir_setting = CircleSetting::new(*draw_pos);

    let path = cir_setting.create_circle_path();

    let color = Color::with_a(Color::YELLOW_GREEN, 0.85);

    let text_style = TextStyle {
        font: res.font.clone(),
        font_size: 15.0,
        color: Color::WHITE,
    };

    let avatar = res.avatars.get(&player_pos).unwrap().clone();
    let score = format!("Score: {}", player_score);

    commands
        .spawn((
            cir_setting,
            Cooldown,
            PlayerPos(player_pos),
            ShapeBundle { path, ..default() },
            // Stroke::new(Color::RED, 2.),
            Fill::color(color),
        ))
        .with_children(|builder| {
            builder.spawn(SpriteBundle {
                texture: avatar,
                transform: Transform::from_xyz(draw_pos.x, draw_pos.y, 0.),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(AVATAR_SIZE, AVATAR_SIZE)),
                    ..default()
                },
                ..default()
            });
        })
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
