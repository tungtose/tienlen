use bevy::prelude::*;
use bevy_mod_picking::prelude::*;
use bevy_tweening::{lens::*, *};
use naia_bevy_client::{events::MessageEvents, Client};
use naia_bevy_demo_shared::{
    channels::{GameSystemChannel, PlayerActionChannel},
    components::{deck::Deck, hand::Hand, rank::Rank, suit::Suit},
    messages::{AcceptPlayCard, AcceptStartGame, EndMatch, PlayCard},
};
use std::{collections::HashMap, ops::Add};

use crate::{
    resources::Global,
    system_set::{Animating, Playing},
};

use super::{
    controller::PlayEvent,
    player_ui::{BackCard, PlayerPos},
    status::DrawStatus,
};

pub struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TweeningPlugin)
            .add_plugins(
                DefaultPickingPlugins.build(), // .disable::<DebugPickingPlugin>(),
            )
            .add_event::<SchedulePileEvent>()
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                send_cards_to_server.in_set(Playing), // .run_if(valid_cards_condition),
            )
            .add_systems(
                Update,
                (
                    handle_accept_play_event,
                    spawn_player_card,
                    update_status,
                    handle_end_match_event,
                    handle_reschedule_pile.in_set(Animating),
                ),
            );
    }
}

#[derive(Event, Clone, Default)]
pub struct SchedulePileEvent(Vec<Entity>);

#[derive(Component)]
struct PlayBtn;

#[derive(Event, Clone)]
struct AnimatingEvent;

#[derive(Component)]
pub struct PlayContainer;

#[derive(Component)]
pub struct Card;

#[derive(Component)]
struct Position(Vec3);

#[derive(Component, Clone)]
pub enum CStatus {
    Idle,
    Active,
    Animating,
}

#[derive(Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ordinal(usize);

impl Ordinal {
    pub fn new(rank: Rank, suilt: Suit) -> Self {
        Self(rank.ordinal() * 13 + suilt.ordinal())
    }

    pub fn get(&self) -> usize {
        self.0
    }
}

#[derive(Component, Clone)]
pub struct Raw(String);

impl Raw {
    pub fn get(&self) -> String {
        self.0.clone()
    }
}

#[derive(Bundle)]
struct CardBundle {
    marker: Card,
    rank: Rank,
    suit: Suit,
    raw: Raw,
    ordinal: Ordinal,
    sprite: SpriteBundle,
}

#[derive(Component)]
pub struct Pile;

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
                    transform: Transform::from_xyz(0., 0., 10.),
                    ..Default::default()
                };

                let ordinal = Ordinal::new(rank, suit);

                let raw = Raw(str);

                let entity = commands
                    .spawn((
                        CardBundle {
                            marker: Card,
                            raw,
                            rank,
                            suit,
                            ordinal,
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
    mut card_q: Query<(&Transform, &mut CStatus, &Ordinal, &mut Visibility), With<Card>>,
) {
    for event in reschedule_pile_ev.read() {
        let mut pile_pos = Vec3::new(0., 0., 10.);
        let mut cards = vec![];

        for c in event.0.iter() {
            let (_trans, status, ordinal, mut vis) = card_q.get_mut(*c).unwrap();
            *vis = Visibility::Visible;
            if let CStatus::Idle = *status {
                cards.push((*c, ordinal.get()));
            }
        }

        cards.sort_by_key(|o| o.1);

        for c in cards.iter().map(|d| d.0) {
            let (trans, mut status, _, _) = card_q.get_mut(c).unwrap();
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

            commands.entity(c).insert(Animator::new(tween));
        }
    }
}

fn send_cards_to_server(
    mut client: Client,
    mut play_event_reader: EventReader<PlayEvent>,
    card_q: Query<&Raw, With<Card>>,
    mut draw_status_ev: EventWriter<DrawStatus>,
) {
    for event in play_event_reader.read() {
        let cards: Vec<String> = event
            .0
            .iter()
            .map(|entity| card_q.get(*entity).unwrap().get())
            .collect();

        let cards = cards.join(",");

        if cards.is_empty() {
            return draw_status_ev
                .send(DrawStatus::Error("You must select at least 1 card".into()));
        }

        let hand = Hand::from_str(&cards);

        if !hand.check_combination() {
            return draw_status_ev
                .send(DrawStatus::Error("Your selected cards is not valid".into()));
        }

        client.send_message::<PlayerActionChannel, PlayCard>(&PlayCard(cards));
    }
}

#[allow(clippy::too_many_arguments)]
#[allow(clippy::type_complexity)]
fn handle_accept_play_event(
    mut commands: Commands,
    global: Res<Global>,
    card_map: Res<CardMap>,
    mut event_reader: EventReader<MessageEvents>,
    mut pile_q: Query<(Entity, &Children), With<Pile>>,
    mut card_q: Query<
        (
            &mut Visibility,
            &GlobalTransform,
            &mut Transform,
            &mut CStatus,
        ),
        With<Card>,
    >,
    back_card_q: Query<(&Transform, &PlayerPos), (With<BackCard>, Without<Card>)>,
    mut reschedule_pile_ev: EventWriter<SchedulePileEvent>,
) {
    for events in event_reader.read() {
        for data in events.read::<GameSystemChannel, AcceptPlayCard>() {
            let mut table_pos = Vec3::new(-150., 50., 10.);
            let cards = card_map.list_from_str(&data.cards);

            if global.game.local_player.pos as usize == data.cur_player {
                let mut need_reschedule = false;
                let Ok((pile_entity, pile_child)) = pile_q.get_single_mut() else {
                    info!("BUG: Not found PILE");
                    return;
                };

                for entity in cards.iter() {
                    need_reschedule = true;
                    let (_, glb_trans, mut trans, mut status) = card_q.get_mut(*entity).unwrap();
                    trans.translation = glb_trans.translation();
                    commands.entity(pile_entity).remove_children(&[*entity]);

                    let tween = Tween::new(
                        EaseFunction::QuadraticInOut,
                        std::time::Duration::from_millis(300),
                        TransformPositionLens {
                            start: glb_trans.translation(),
                            end: table_pos,
                        },
                    )
                    .with_completed_event(2);

                    table_pos.x += 35.;

                    commands.entity(*entity).insert(Animator::new(tween));

                    *status = CStatus::Animating;
                }

                if need_reschedule {
                    let cards = pile_child.iter().copied().collect::<Vec<Entity>>();
                    reschedule_pile_ev.send(SchedulePileEvent(cards));
                }
            } else {
                // Foregin player play card animation
                for entity in cards.iter() {
                    let mut card = card_q.get_mut(*entity).unwrap();

                    *card.0 = Visibility::Visible;

                    let mut back_card_pos = Vec3::splat(0.);

                    for (trans, p_pos) in back_card_q.iter() {
                        if p_pos.0 == data.cur_player as i32 {
                            back_card_pos = trans.translation;
                        }
                    }

                    let tween = Tween::new(
                        EaseFunction::QuadraticIn,
                        std::time::Duration::from_millis(300),
                        TransformPositionLens {
                            start: back_card_pos,
                            end: table_pos,
                        },
                    )
                    .with_completed_event(2);

                    table_pos.x += 35.;

                    commands.entity(*entity).insert(Animator::new(tween));
                }
            }
        }
    }
}

fn update_status(
    mut query_event: EventReader<TweenCompleted>,
    mut card_q: Query<&mut CStatus, With<Card>>,
) {
    for ev in query_event.read() {
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
                    end: tran.translation.add(Vec3::new(0., 15., 0.)),
                },
            )
            .with_completed_event(0);

            commands.entity(event.target).insert(Animator::new(tween));
            *status = CStatus::Animating;
        }
        CStatus::Active => {
            let tween = Tween::new(
                EaseFunction::CubicIn,
                std::time::Duration::from_millis(100),
                TransformPositionLens {
                    start: tran.translation,
                    end: tran.translation.add(Vec3::new(0., -15., 0.)),
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

impl CardMap {
    pub fn list_from_str(&self, input: &str) -> Vec<Entity> {
        input
            .split(',')
            .map(|c| *self.0.get(c).unwrap())
            .collect::<Vec<Entity>>()
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut desk = Deck::new();
    let mut card_map = CardMap::default();

    for c in desk.deal(52).as_slice() {
        let card_entity = CardBundle::from_str(&c.to_str(), &mut commands, &asset_server).unwrap();
        card_map.0.insert(c.to_str(), card_entity);
    }

    commands.insert_resource(card_map);
}

fn spawn_player_card(
    mut commands: Commands,
    card_map: Res<CardMap>,
    mut event_reader: EventReader<MessageEvents>,
    mut schedule_pile_event: EventWriter<SchedulePileEvent>,
    mut card_q: Query<&mut Visibility, With<Card>>,
    global: Res<Global>,
) {
    for events in event_reader.read() {
        for message in events.read::<GameSystemChannel, AcceptStartGame>() {
            info!("GOT ACCEPT START GAME!");
            let cards: Vec<Entity> = card_map.list_from_str(&message.cards);

            for c in cards.iter() {
                let mut vis = card_q.get_mut(*c).unwrap();
                *vis = Visibility::Visible;
            }

            commands
                .spawn((
                    SpatialBundle {
                        transform: Transform::from_translation(global.game.local_player.pile_pos),
                        ..Default::default()
                    },
                    Pile,
                ))
                .push_children(&cards);

            schedule_pile_event.send(SchedulePileEvent(cards));
        }
    }
}

pub fn handle_end_match_event(
    mut event_reader: EventReader<MessageEvents>,
    mut pile_q: Query<Entity, With<Pile>>,
    mut card_q: Query<&mut Visibility, With<Card>>,
    mut commands: Commands,
) {
    for event in event_reader.read() {
        for _end_match in event.read::<GameSystemChannel, EndMatch>() {
            for mut old_card_vis in card_q.iter_mut() {
                *old_card_vis = Visibility::Hidden;
            }

            for e in pile_q.iter_mut() {
                commands.entity(e).clear_children();
                commands.entity(e).despawn();
            }
        }
    }
}
