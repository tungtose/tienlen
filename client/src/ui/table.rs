use std::time::Duration;

use bevy::prelude::*;
use naia_bevy_demo_shared::components::hand::Hand;

const DECK_HEIGHT: f32 = 50.;
const CARD_WIDTH: f32 = 32.;
const CARD_HEIGHT: f32 = 48.;
const CARD_MARGIN: f32 = 2.;

use crate::resources::Global;

use super::{DrawStatus, UiAssets};

#[derive(Component)]
pub struct TableBg;

#[derive(Component)]
pub struct TableContainer;

#[derive(Component)]
pub struct TableCardContainer;

#[derive(Component, Default)]
pub struct TableCards {
    pub cards: Vec<Entity>,
}

#[derive(Component)]
pub struct TableCard;

#[derive(Component)]
pub struct StatusContainer;

#[derive(Component)]
pub struct CounterConfig {
    timer: Timer,
}

pub fn delete_status(
    mut commands: Commands,
    mut counter_q: Query<(Entity, &mut CounterConfig)>,
    status_container_q: Query<Entity, With<StatusContainer>>,
    time: Res<Time>,
) {
    for (entity, mut counter) in counter_q.iter_mut() {
        counter.timer.tick(time.delta());

        if counter.timer.finished() {
            commands.entity(entity).despawn();

            let status_container = status_container_q.get_single().unwrap();

            commands.entity(status_container).despawn_descendants();
        }
    }
}

pub fn draw_status(
    mut commands: Commands,
    mut status_ev: EventReader<DrawStatus>,
    status_container_q: Query<Entity, With<StatusContainer>>,
    res: Res<UiAssets>,
) {
    let status_container = status_container_q.get_single().unwrap();

    for d_status in status_ev.iter() {
        let msg = d_status.0.clone();

        let status_text = commands
            .spawn(TextBundle::from_section(
                msg,
                TextStyle {
                    font: res.font.clone(),
                    font_size: 16.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ))
            .id();

        commands.entity(status_container).add_child(status_text);
        commands.spawn(CounterConfig {
            timer: Timer::new(Duration::from_secs(3), TimerMode::Once),
        });
    }
}

pub fn get_card_button(commands: &mut Commands, image: &Handle<Image>) -> Entity {
    commands
        .spawn((
            TableCard,
            ImageBundle {
                style: Style {
                    width: Val::Px(CARD_WIDTH),
                    height: Val::Px(CARD_HEIGHT),
                    margin: UiRect::all(Val::Px(CARD_MARGIN)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                image: UiImage::new(image.clone()),
                ..Default::default()
            },
        ))
        .id()
}

pub fn draw_table(
    mut commands: Commands,
    table_card_container_query: Query<Entity, With<TableCardContainer>>,
    res: Res<UiAssets>,
    global: Res<Global>,
) {
    clear_table_cards(&mut commands, &table_card_container_query);

    let Ok(table_card_container) = table_card_container_query.get_single() else {
        return;
    };

    if global.game.table_cards.is_empty() {
        return;
    }

    let hand = Hand::from_str(&global.game.table_cards);

    for card in hand.cards {
        let handle = res.cards.get(&card.name()).unwrap();
        let card_ui = get_card_button(&mut commands, handle);

        commands.entity(table_card_container).add_child(card_ui);
    }
}

pub fn clear_table_cards(
    commands: &mut Commands,
    table_container_query: &Query<Entity, With<TableCardContainer>>,
) {
    for entity in table_container_query.iter() {
        commands.entity(entity).despawn_descendants();
    }
}

pub fn spawn_table(mut commands: Commands, assets: Res<UiAssets>) {
    let cards_container = commands
        .spawn((
            TableCardContainer,
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Px(DECK_HEIGHT),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let status_container = commands
        .spawn((
            StatusContainer,
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    width: Val::Percent(100.),
                    height: Val::Px(DECK_HEIGHT),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let table_bg = commands
        .spawn(ImageBundle {
            image: UiImage::new(assets.board.clone()),
            style: Style {
                position_type: PositionType::Relative,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                width: Val::Px(600.),
                height: Val::Px(300.),
                padding: UiRect::bottom(Val::Px(100.)),
                ..Default::default()
            },
            visibility: Visibility::Hidden,
            ..default()
        })
        .add_child(status_container)
        .add_child(cards_container)
        .id();

    commands
        .spawn((
            TableContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    top: Val::Px(80.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.),
                    height: Val::Px(300.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .add_child(table_bg)
        .insert(TableContainer);

    let table_cards = TableCards { cards: vec![] };
    commands.spawn(table_cards);
}
