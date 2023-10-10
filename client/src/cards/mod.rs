use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::*, *};
use naia_bevy_demo_shared::components::{deck::Deck, rank::Rank, suit::Suit};
use std::{collections::HashMap, ops::Add};

use crate::system_set::{Animating, Playing};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_plugins(DefaultPickingPlugins)
            .add_event::<SchedulePileEvent>()
            .add_event::<SpawnPlayerCardEvent>()
            .add_event::<PlayEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    player_btn_click,
                    spawn_player_card,
                    update_status,
                    handle_play_event.in_set(Playing),
                    handle_reschedule_pile.in_set(Animating),
                ),
            );
    }
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);

#[derive(Event, Clone, Default)]
struct SchedulePileEvent(Vec<Entity>);

#[derive(Event, Clone, Default)]
pub struct SpawnPlayerCardEvent(pub String);

#[derive(Event, Clone, Default)]
struct PlayEvent;

#[derive(Component)]
struct PlayBtn;

#[derive(Event, Clone)]
struct AnimatingEvent;

#[derive(Component)]
struct Card;

#[derive(Component)]
struct Position(Vec3);

#[derive(Component, Clone)]
enum CStatus {
    Idle,
    Active,
    Animating,
}

#[derive(Bundle)]
struct CardBundle {
    marker: Card,
    rank: Rank,
    suit: Suit,
    sprite: SpriteBundle,
}

#[derive(Component)]
struct Pile;

impl CardBundle {
    pub fn from_str(
        s: &str,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) -> Result<Entity, Box<dyn std::error::Error>> {
        if s.len() != 2 {
            return Err("Card string must be length equal to 2".into());
        }

        let str = s.to_string();
        let mut char = str.chars();
        let char_rank = char.next().unwrap();
        let char_suit = char.next().unwrap();

        if let Ok(rank) = Rank::from_char(char_rank) {
            if let Ok(suit) = Suit::from_char(char_suit) {
                let asset_path = format!(
                    "cards/standard/solitaire/individuals/{}/{}.png",
                    suit.get_asset_path(),
                    rank.get_asset_path()
                );

                info!("Path: {}", asset_path);

                let sprite = SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(30., 45.)),
                        ..Default::default()
                    },
                    texture: asset_server.load(asset_path),
                    visibility: Visibility::Hidden,
                    transform: Transform::from_xyz(0., 0., 0.),
                    ..Default::default()
                };

                let entity = commands
                    .spawn((
                        CardBundle {
                            marker: Card,
                            rank,
                            suit,
                            sprite,
                        },
                        CStatus::Idle,
                        On::<Pointer<Click>>::run(click_card),
                    ))
                    .id();

                return Ok(entity);
            }
        }

        Err("unexpected error".into())
    }
}

fn handle_reschedule_pile(
    mut commands: Commands,
    mut reschedule_pile_ev: EventReader<SchedulePileEvent>,
    pile_q: Query<&Children, With<Pile>>,
    mut card_q: Query<(&Transform, &mut CStatus), With<Card>>,
) {
    for event in reschedule_pile_ev.iter() {
        info!("GOT EVENT! {:?}", pile_q.iter().len());
        let mut pile_pos = Vec3::new(0., 0., 0.);
        for c in event.0.iter() {
            info!("GoT CHILD");
            let (trans, mut status) = card_q.get_mut(*c).unwrap();
            if let CStatus::Idle = *status {
                info!("==== GoT Idle");
                let tween = Tween::new(
                    EaseFunction::QuarticIn,
                    std::time::Duration::from_millis(300),
                    TransformPositionLens {
                        start: trans.translation,
                        end: pile_pos,
                    },
                )
                .with_completed_event(4);

                pile_pos.x += 35.;

                *status = CStatus::Animating;

                commands.entity(*c).insert(Animator::new(tween));
            }
        }
    }
}

fn handle_play_event(
    mut commands: Commands,
    mut play_event_reader: EventReader<PlayEvent>,
    mut active_card_q: Query<
        (&mut CStatus, &mut Transform, &mut GlobalTransform, Entity),
        With<Card>,
    >,
    mut pile_q: Query<(Entity, &Children), With<Pile>>,
    mut reschedule_pile_ev: EventWriter<SchedulePileEvent>,
) {
    for _ev in play_event_reader.iter() {
        let mut need_reschedule = false;
        let (pile_entity, pile_child) = pile_q.get_single_mut().unwrap();
        let mut table_pos = Vec3::new(-150., 10., 0.);
        let mut active_cards = vec![];
        for (mut status, mut trans, glb_trans, entity) in active_card_q.iter_mut() {
            if let CStatus::Active = *status {
                need_reschedule = true;

                active_cards.push(entity);

                trans.translation = glb_trans.translation();
                commands.entity(pile_entity).remove_children(&[entity]);

                let tween = Tween::new(
                    EaseFunction::CubicIn,
                    std::time::Duration::from_millis(150),
                    TransformPositionLens {
                        start: glb_trans.translation(),
                        end: table_pos,
                    },
                )
                .with_completed_event(2);

                table_pos.x += 35.;

                commands.entity(entity).insert(Animator::new(tween));

                *status = CStatus::Animating;
            }
        }
        if need_reschedule {
            let cards = pile_child.iter().copied().collect::<Vec<Entity>>();
            reschedule_pile_ev.send(SchedulePileEvent(cards));
        }
    }
}

#[allow(clippy::type_complexity)]
fn player_btn_click(
    mut interaction_query: Query<
        (&Interaction, Option<&PlayBtn>),
        (Changed<Interaction>, With<Button>),
    >,
    mut play_event_writer: EventWriter<PlayEvent>,
) {
    for (interaction, play_btn) in &mut interaction_query {
        if let Interaction::Pressed = *interaction {
            if play_btn.is_some() {
                info!("Clicked play!");
                play_event_writer.send_default();
            }
        }
    }
}

fn update_status(
    mut query_event: EventReader<TweenCompleted>,
    mut card_q: Query<&mut CStatus, With<Card>>,
) {
    for ev in query_event.iter() {
        let mut status = card_q.get_mut(ev.entity).unwrap();

        if ev.user_data == 0 {
            *status = CStatus::Active;
        } else {
            *status = CStatus::Idle;
        }
    }
}

fn click_card(
    event: Listener<Pointer<Click>>,
    mut commands: Commands,
    mut card_q: Query<(&mut Transform, &mut CStatus), With<Card>>,
) {
    let (tran, mut status) = card_q.get_mut(event.target).unwrap();

    match *status {
        CStatus::Idle => {
            let tween = Tween::new(
                EaseFunction::CubicIn,
                std::time::Duration::from_millis(100),
                TransformPositionLens {
                    start: tran.translation,
                    end: tran.translation.add(Vec3::new(0., 10., 0.)),
                },
            )
            .with_completed_event(0);

            commands.entity(event.target).insert(Animator::new(tween));
            *status = CStatus::Animating;
        }
        CStatus::Active => {
            let tween = Tween::new(
                EaseFunction::CubicIn,
                std::time::Duration::from_millis(200),
                TransformPositionLens {
                    start: tran.translation,
                    end: tran.translation.add(Vec3::new(0., -10., 0.)),
                },
            )
            .with_completed_event(1);

            commands.entity(event.target).insert(Animator::new(tween));
            *status = CStatus::Animating;
        }
        _ => {}
    }
}

#[derive(Resource, Default)]
pub struct CardMap(pub HashMap<String, Entity>);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut desk = Deck::new();
    let mut card_map = CardMap::default();

    for c in desk.deal(52).as_slice() {
        info!(" C STR = {:?}", c.to_str());
        let card_entity = CardBundle::from_str(&c.to_str(), &mut commands, &asset_server).unwrap();
        card_map.0.insert(c.to_str(), card_entity);
    }

    commands.insert_resource(card_map);
}

fn spawn_player_card(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    card_map: Res<CardMap>,
    mut spawn_card_event: EventReader<SpawnPlayerCardEvent>,
    mut schedule_pile_event: EventWriter<SchedulePileEvent>,
    mut card_q: Query<&mut Visibility, With<Card>>,
) {
    for ev in spawn_card_event.iter() {
        let cards: Vec<Entity> =
            ev.0.split(',')
                .map(|card| *card_map.0.get(card).unwrap())
                .collect();

        for c in cards.iter() {
            let mut vis = card_q.get_mut(*c).unwrap();
            *vis = Visibility::Visible;
        }

        commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(-100., -150., 0.),
                    ..Default::default()
                },
                Pile,
            ))
            .push_children(&cards);

        let _play_btn = commands
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
                                font_size: 16.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                                ..Default::default()
                            },
                        ));
                    })
                    .insert(PlayBtn);
            });

        schedule_pile_event.send(SchedulePileEvent(cards));
    }
}
