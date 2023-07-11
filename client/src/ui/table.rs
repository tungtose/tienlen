use std::time::Duration;

use bevy::{prelude::*, window::PrimaryWindow};
use naia_bevy_demo_shared::components::{hand::Hand, Table};

const DECK_HEIGHT: f32 = 50.;
const CARD_WIDTH: f32 = 32.;
const CARD_HEIGHT: f32 = 48.;
const CARD_MARGIN: f32 = 2.;

use super::{DrawPlayer, DrawStatus, UiAssets};

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
pub struct Status(pub String);

#[derive(Component)]
pub struct CounterConfig {
    /// How often to spawn a new bomb? (repeating timer)
    timer: Timer,
}

impl Status {
    fn make_empty(&mut self) {
        self.0.clear();
    }
}

impl Default for Status {
    fn default() -> Self {
        Self("".to_string())
    }
}

pub fn delete_status(
    mut commands: Commands,
    // mut status_q: Query<&mut Status>,
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
    // mut status_q: Query<&mut Status>,
    mut status_ev: EventReader<DrawStatus>,
    status_container_q: Query<Entity, With<StatusContainer>>,
    res: Res<UiAssets>,
) {
    // let status = status_q.get_single_mut().unwrap();
    //
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
    assets: Res<UiAssets>,
    mut draw_player_ev: EventWriter<DrawPlayer>,
) {
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

    let status_container = commands
        .spawn((
            StatusContainer,
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
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
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                size: Size::new(Val::Px(600.), Val::Px(300.)),
                padding: UiRect::bottom(Val::Px(100.)),
                ..Default::default()
            },
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
                    position: UiRect::top(Val::Px(80.)),
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
    let status = Status::default();

    commands.spawn(status);
    commands.spawn(table_cards);

    draw_player_ev.send(DrawPlayer);
    info!("SPAWNED TABLE!!!");
}
