use bevy::{prelude::*, window::PrimaryWindow};
use naia_bevy_demo_shared::components::{card::Card, hand::Hand, Table};

use crate::resources::Global;

use super::UiAssets;

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

const DECK_HEIGHT: f32 = 50.;
const CARD_WIDTH: f32 = 32.;
const CARD_HEIGHT: f32 = 48.;
const CARD_MARGIN: f32 = 2.;

pub fn get_card_button(commands: &mut Commands, size: Size, image: &Handle<Image>) -> Entity {
    commands
        .spawn((
            TableCard,
            ImageBundle {
                style: Style {
                    size,
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
    server_table_q: Query<&Table>,
) {
    clear_table_cards(&mut commands, &table_card_container_query);

    let Ok(table_card_container) = table_card_container_query.get_single() else {
        return;
    };

    let table_server = server_table_q.get_single();

    let table_str: String;

    match table_server {
        Ok(table) => {
            table_str = table.cards.to_string();
        }
        _ => {
            table_str = "".to_string();
        }
    }

    if table_str.is_empty() {
        return;
    }

    let hand = Hand::from_str(&table_str);

    for card in hand.cards {
        let handle = res.cards.get(&card.name()).unwrap();
        let card_ui = get_card_button(
            &mut commands,
            Size::new(Val::Px(CARD_WIDTH), Val::Px(CARD_HEIGHT)),
            handle,
        );

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

pub fn spawn_table(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    assets: Res<UiAssets>,
) {
    let window = window_query.get_single().unwrap();

    let cards_container = commands
        .spawn((
            TableCardContainer,
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.), Val::Px(DECK_HEIGHT)),
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
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(600.), Val::Px(300.)),
                ..Default::default()
            },
            ..default()
        })
        .add_child(cards_container)
        .id();

    commands
        .spawn((
            TableContainer,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect::bottom(Val::Px(window.height() / 2.)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Percent(100.), Val::Px(300.)),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .add_child(table_bg)
        .insert(TableContainer);

    let table_cards = TableCards { cards: vec![] };

    commands.spawn(table_cards);

    info!("SPAWNED TABLE!!!");
}
