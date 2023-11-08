use std::{ops::Add, time::Duration};

use bevy::time::common_conditions::on_timer;
use bevy::{prelude::*, text::Text2dBounds};
use naia_bevy_client::events::MessageEvents;
use naia_bevy_demo_shared::{channels::GameSystemChannel, messages::AcceptPlayerReady};

use naia_bevy_demo_shared::components::Player;

use crate::{assets::UiAssets, components::LocalPlayer, resources::Global, states::MainState};

pub struct PlayerUiPlugin;

impl Plugin for PlayerUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerMessageEvent>()
            .add_event::<LoadExistPlayerEvent>()
            .add_systems(
                Update,
                (new_player_join, handle_load_exist_player).run_if(in_state(MainState::Lobby)),
            )
            .add_systems(
                Update,
                (
                    clean_player_message,
                    update_player_message,
                    update_timer,
                    update_score,
                )
                    .run_if(in_state(MainState::Game)),
            )
            .add_systems(
                Update,
                animatetext_update.run_if(
                    in_state(MainState::Game).and_then(on_timer(Duration::from_millis(800))),
                ),
            );
    }
}

const AVATAR_SIZE: f32 = 55.;

#[derive(Event, Default)]
pub struct LoadExistPlayerEvent(pub usize);

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
pub struct Playing;

#[derive(Component)]
pub struct PlayerUiMarker;

#[derive(Component)]
pub struct CleanMessageCounter {
    timer: Timer,
    pos: i32,
}

#[derive(Component, Copy, Clone)]
pub struct Bottom(usize);

#[derive(Component, Copy, Clone)]
pub struct Left(usize);

#[derive(Component, Copy, Clone)]
pub struct Top(usize);

#[derive(Component, Copy, Clone)]
pub struct Right(usize);

pub trait PlayerDirection {
    fn from_server_pos(pos: usize) -> Self;
    fn get_translation(&self) -> Vec3;
    fn back_card_translation(&self) -> Vec3;
    fn timer_translation(&self) -> Vec3 {
        self.get_translation().add(Vec3::new(50., 0., 15.))
    }
}

impl PlayerDirection for Bottom {
    fn from_server_pos(pos: usize) -> Self {
        Self(pos)
    }

    fn get_translation(&self) -> Vec3 {
        Vec3::new(0., -180., 5.)
    }

    fn back_card_translation(&self) -> Vec3 {
        self.get_translation().add(Vec3::new(-1000., 0., 0.))
    }
}

impl PlayerDirection for Left {
    fn get_translation(&self) -> Vec3 {
        Vec3::new(-315., 45., 5.)
    }

    fn from_server_pos(pos: usize) -> Self {
        Self(pos)
    }

    fn back_card_translation(&self) -> Vec3 {
        self.get_translation().add(Vec3::new(60., 0., 0.))
    }

    fn timer_translation(&self) -> Vec3 {
        self.get_translation().add(Vec3::new(-50., 0., 15.))
    }
}

impl PlayerDirection for Top {
    fn get_translation(&self) -> Vec3 {
        Vec3::new(0., 200., 5.)
    }

    fn from_server_pos(pos: usize) -> Self {
        Self(pos)
    }

    fn back_card_translation(&self) -> Vec3 {
        self.get_translation().add(Vec3::new(60., 0., 0.))
    }

    fn timer_translation(&self) -> Vec3 {
        self.get_translation().add(Vec3::new(-50., 0., 15.))
    }
}

impl PlayerDirection for Right {
    fn get_translation(&self) -> Vec3 {
        Vec3::new(315., 45., 5.)
    }

    fn from_server_pos(pos: usize) -> Self {
        Self(pos)
    }

    fn back_card_translation(&self) -> Vec3 {
        self.get_translation().add(Vec3::new(-50., 0., 0.))
    }
}

pub fn handle_load_exist_player(
    mut commands: Commands,
    res: Res<UiAssets>,
    mut event_reader: EventReader<LoadExistPlayerEvent>,
    player_q: Query<&Player, Without<LocalPlayer>>,
) {
    for event in event_reader.iter() {
        let cur_local_pos = event.0;

        if cur_local_pos == 0 {
            return;
        }

        if cur_local_pos == 1 {
            for p in player_q.iter() {
                let right = Right::from_server_pos(*p.pos);

                let entity = create_player_ui(
                    &mut commands,
                    right,
                    &res,
                    *p.pos as i32,
                    &p.score.to_string(),
                    &p.name.to_string(),
                );

                commands.entity(entity).insert(right);
            }
        }

        if cur_local_pos == 2 {
            for p in player_q.iter() {
                if *p.pos == 0 {
                    let top = Top::from_server_pos(*p.pos);
                    let entity = create_player_ui(
                        &mut commands,
                        top,
                        &res,
                        *p.pos as i32,
                        &p.score.to_string(),
                        &p.name.to_string(),
                    );

                    commands.entity(entity).insert(top);
                }

                if *p.pos == 1 {
                    let right = Right::from_server_pos(*p.pos);
                    let entity = create_player_ui(
                        &mut commands,
                        right,
                        &res,
                        *p.pos as i32,
                        &p.score.to_string(),
                        &p.name.to_string(),
                    );

                    commands.entity(entity).insert(right);
                }
            }
        }

        if cur_local_pos == 3 {
            for p in player_q.iter() {
                if *p.pos == 0 {
                    let left = Left::from_server_pos(*p.pos);
                    let entity = create_player_ui(
                        &mut commands,
                        left,
                        &res,
                        *p.pos as i32,
                        &p.score.to_string(),
                        &p.name.to_string(),
                    );

                    commands.entity(entity).insert(left);
                }

                if *p.pos == 2 {
                    let right = Right::from_server_pos(*p.pos);
                    let entity = create_player_ui(
                        &mut commands,
                        right,
                        &res,
                        *p.pos as i32,
                        &p.score.to_string(),
                        &p.name.to_string(),
                    );

                    commands.entity(entity).insert(right);
                }

                if *p.pos == 1 {
                    let top = Top::from_server_pos(*p.pos);
                    let entity = create_player_ui(
                        &mut commands,
                        top,
                        &res,
                        *p.pos as i32,
                        &p.score.to_string(),
                        &p.name.to_string(),
                    );

                    commands.entity(entity).insert(top);
                }
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn new_player_join(
    mut commands: Commands,
    res: Res<UiAssets>,
    mut event_reader: EventReader<MessageEvents>,
    mut load_exist_player_event: EventWriter<LoadExistPlayerEvent>,
    bottom_player_q: Query<(), With<Bottom>>,
    left_player_q: Query<(), With<Left>>,
    top_player_q: Query<(), With<Top>>,
    right_player_q: Query<(), With<Right>>,
) {
    for events in event_reader.iter() {
        for new_player in events.read::<GameSystemChannel, AcceptPlayerReady>() {
            let player_name = new_player.name;
            let player_pos = new_player.server_pos as i32;
            let player_score = "0";

            if bottom_player_q.is_empty() {
                let bottom = Bottom::from_server_pos(player_pos as usize);

                let entity = create_player_ui(
                    &mut commands,
                    bottom,
                    &res,
                    player_pos,
                    player_score,
                    &player_name,
                );

                commands.entity(entity).insert(bottom);

                load_exist_player_event.send(LoadExistPlayerEvent(new_player.server_pos));

                return;
            }

            if left_player_q.is_empty() {
                let left = Left::from_server_pos(player_pos as usize);

                let entity = create_player_ui(
                    &mut commands,
                    left,
                    &res,
                    player_pos,
                    player_score,
                    &player_name,
                );

                commands.entity(entity).insert(left);

                return;
            }

            if top_player_q.is_empty() {
                let top = Top::from_server_pos(player_pos as usize);

                let entity = create_player_ui(
                    &mut commands,
                    top,
                    &res,
                    player_pos,
                    player_score,
                    &player_name,
                );

                commands.entity(entity).insert(top);

                return;
            }

            if right_player_q.is_empty() {
                let right = Right::from_server_pos(player_pos as usize);

                let entity = create_player_ui(
                    &mut commands,
                    right,
                    &res,
                    player_pos,
                    player_score,
                    &player_name,
                );

                commands.entity(entity).insert(right);

                return;
            }
        }
    }
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
        font: res.noto_font.clone(),
        font_size: 16.,
        color: Color::ORANGE_RED,
    };

    let normal_style = TextStyle {
        font: res.font.clone(),
        font_size: 16.,
        color: Color::ORANGE_RED,
    };

    for (mut vis, player_pos, mut text) in text_q.iter_mut() {
        if player_pos.0 == global.game.active_player_pos {
            let clock = TextSection::new("⏰", timer_text.clone());
            let time = TextSection::new(&global.game.timer, normal_style.clone());
            *text = Text::from_sections([clock, time]);
            *vis = Visibility::Visible;
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
        let name = text.sections.first().unwrap().value.clone();
        if player_pos.0 == global.game.active_player_pos {
            *text = Text::from_section(name, playing_text.clone());
            match *vis {
                Visibility::Hidden => *vis = Visibility::Visible,
                Visibility::Visible => *vis = Visibility::Hidden,
                Visibility::Inherited => *vis = Visibility::Visible,
            }

            continue;
        }

        *text = Text::from_section(name, idle_text.clone());

        *vis = Visibility::Visible;
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

pub fn create_player_ui<T: PlayerDirection>(
    commands: &mut Commands,
    direction: T,
    res: &Res<UiAssets>,
    player_pos: i32,
    player_score: &str,
    player_name: &str,
) -> Entity {
    let draw_pos = direction.get_translation();

    let text_style = TextStyle {
        font: res.font.clone(),
        font_size: 15.0,
        color: Color::WHITE,
    };

    let avatar_handle = res.avatars.get(&player_pos).unwrap().clone();
    let score = format!("Score: {}", player_score);

    let avatar = SpriteBundle {
        transform: Transform::from_xyz(draw_pos.x, draw_pos.y, draw_pos.z),
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
            transform: Transform::from_translation(direction.timer_translation()),
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

    commands.spawn((
        BackCard,
        PlayerPos(player_pos),
        SpriteBundle {
            texture: back_card_handle,
            sprite: Sprite {
                custom_size: Some(Vec2::new(30., 45.)),
                ..default()
            },
            transform: Transform::from_translation(direction.back_card_translation()),
            ..default()
        },
    ));

    let entity = commands.spawn((avatar, PlayerPos(player_pos))).id();

    entity
}
