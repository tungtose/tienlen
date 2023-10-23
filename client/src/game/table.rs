use bevy::prelude::*;
use std::collections::VecDeque;

use naia_bevy_client::events::MessageEvents;
use naia_bevy_demo_shared::{channels::GameSystemChannel, messages::AcceptPlayCard};

use crate::{states::MainState, ui::UiAssets};

use super::cards::{Card, CardMap};

pub struct TablePlugin;

impl Plugin for TablePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(MainState::Lobby), setup)
            .add_systems(Update, handle_accept_play_event);
    }
}

#[derive(Component)]
pub struct Table;

#[derive(Component)]
pub struct TablePile(VecDeque<Vec<Entity>>);

fn handle_accept_play_event(
    card_map: Res<CardMap>,
    mut event_reader: EventReader<MessageEvents>,
    mut card_q: Query<&mut Visibility, With<Card>>,
    mut table_pile_q: Query<&mut TablePile>,
) {
    for events in event_reader.iter() {
        for data in events.read::<GameSystemChannel, AcceptPlayCard>() {
            let mut table_pile = table_pile_q.get_single_mut().unwrap();
            if let Some(pile) = table_pile.0.back() {
                for entity in pile.iter() {
                    let mut vis = card_q.get_mut(*entity).unwrap();
                    *vis = Visibility::Hidden;
                }
            }

            let new_pile = card_map.list_from_str(&data.cards);

            table_pile.0.push_back(new_pile);
        }
    }
}

pub fn setup(mut commands: Commands, res: Res<UiAssets>) {
    let table = commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0., 0., 0.),
                ..Default::default()
            },
            Table,
        ))
        .id();

    let sprite = commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(600., 300.)),
                ..Default::default()
            },
            texture: res.board.clone(),
            transform: Transform::from_xyz(0., 50., 3.),
            ..Default::default()
        })
        .id();

    commands.entity(table).add_child(sprite);

    commands.spawn(TablePile(VecDeque::new()));
}
