use std::collections::HashSet;

use bevy::prelude::*;
use naia_bevy_demo_shared::components::{card::Card, server_hand::ServerHand};

use crate::{components::LocalPlayer, resources::Global, ui::DrawPlayer};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LocalStartGameEvent>()
            .add_event::<PlayerEvent>()
            .add_event::<SelectCardEvent>()
            .add_startup_system(local_init)
            .add_system(spawn_player.run_if(on_event::<LocalStartGameEvent>()))
            .add_system(play_card.run_if(on_event::<PlayerEvent>()))
            .add_system(select_card.run_if(on_event::<SelectCardEvent>()));
        // .add_system(active_cards.run_if(on_event::<PlayerEvent>()));
    }
}

pub struct LocalStartGameEvent;

pub struct SelectCardEvent(pub Entity);

pub enum PlayerEventKind {
    Play,
    Skip,
}

pub struct PlayerEvent(pub PlayerEventKind);

#[derive(Component)]
pub struct ActiveCard(pub bool);

#[derive(Component)]
pub struct ActiveCards(HashSet<Card>);

impl ActiveCards {
    pub fn is_active(&self, card: &Card) -> bool {
        self.0.contains(card)
    }
}

fn local_init(mut commands: Commands) {
    commands.spawn(ActiveCards(HashSet::new()));
}

pub fn select_card(
    mut active_cards_q: Query<&mut ActiveCards>,
    mut card_q: Query<&Card>,
    mut draw_player_ev: EventWriter<DrawPlayer>,
    mut select_card_ev: EventReader<SelectCardEvent>,
) {
    for event in select_card_ev.iter() {
        let entity = event.0;
        if let Ok(card) = card_q.get(entity) {
            let mut active_cards = active_cards_q.get_single_mut().unwrap();

            if active_cards.0.contains(card) {
                active_cards.0.remove(card);
            } else {
                active_cards.0.insert(*card);
            }
        }
        draw_player_ev.send(DrawPlayer);
    }
}

pub fn play_card(mut commands: Commands, active_cards: Query<&ActiveCards>) {
    // let active_cards = card_q
    //     .iter()
    //     .map(|(card, active)| (card.to_str(), active.0))
    //     .filter(|(_card, is_active)| *is_active)
    //     .map(|(card, _is_active)| card)
    //     .collect::<Vec<String>>();

    info!("IAM INTO PLAY");
    for ac in active_cards.iter() {
        info!("PLAY: {:?}", ac.0);
    }
    // info!("DBG Cards: {:?}", ac.0);
}

pub fn spawn_player(
    mut commands: Commands,
    hand_q: Query<&ServerHand, With<LocalPlayer>>,
    player_q: Query<Entity, With<LocalPlayer>>,
    mut global: ResMut<Global>,
    mut draw_player_ev: EventWriter<DrawPlayer>,
) {
    let hand_str = hand_q
        .get_single()
        .unwrap()
        .cards
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    let sl: Vec<&str> = hand_str.iter().map(|str| str.as_str()).collect();

    let player_entity = player_q.get_single().unwrap();

    info!("cards: {:?}", sl);

    for card_str in sl {
        let card_rs = Card::from_str(card_str);

        if let Ok(card) = card_rs {
            info!("Inserting cards: {}", card.to_str());
            let card_entity = commands.spawn(card).id();
            global.player_cards.push(card_entity);
            commands.entity(player_entity).add_child(card_entity);
        } else {
            info!("SPAWN CARD ERROR: {}", card_str);
        }
    }

    draw_player_ev.send(DrawPlayer);

    info!("Done Spawn player");
}
