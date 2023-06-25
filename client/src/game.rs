use std::collections::{HashMap, HashSet};

use bevy::prelude::*;
use naia_bevy_client::Client;
use naia_bevy_demo_shared::{
    channels::PlayerActionChannel,
    components::{card::Card, hand::Hand, server_hand::ServerHand},
    messages::PlayCard,
};

use crate::{components::LocalPlayer, resources::Global, ui::DrawPlayer};

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LocalStartGame>()
            .add_event::<PlayerEvent>()
            .add_event::<SelectCardEvent>()
            .add_startup_system(local_init)
            .add_system(spawn_player.run_if(on_event::<LocalStartGame>()))
            .add_system(play_card.run_if(on_event::<PlayerEvent>()))
            .add_system(select_card.run_if(on_event::<SelectCardEvent>()));
        // .add_system(active_cards.run_if(on_event::<PlayerEvent>()));
    }
}

pub struct LocalStartGame;

pub struct SelectCardEvent(pub Entity);

pub enum PlayerEventKind {
    Play,
    Skip,
}

pub struct PlayerEvent(pub PlayerEventKind);

#[derive(Component)]
pub struct ActiveCard(pub bool);

#[derive(Component)]
pub struct ActiveCards(HashMap<Entity, Card>);

impl ActiveCards {
    pub fn is_active(&self, entity: &Entity) -> bool {
        self.0.contains_key(entity)
    }
    pub fn make_active(&mut self, entity: &Entity, card: &Card) {
        if self.0.contains_key(entity) {
            self.0.remove(entity);
        } else {
            self.0.insert(*entity, *card);
        }
    }

    pub fn to_vec(&mut self) -> Vec<Card> {
        let cards = {
            let mut cards_vec = vec![];
            for card in self.0.values() {
                cards_vec.push(*card);
            }
            cards_vec
        };
        cards
    }

    pub fn to_string(&mut self) -> String {
        self.to_vec()
            .iter()
            .map(|c| c.to_str())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn to_hand(&mut self) -> Hand {
        let cards = self.to_vec();
        self.clear();

        Hand { cards }
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }
}

fn local_init(mut commands: Commands) {
    commands.spawn(ActiveCards(HashMap::new()));
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
            active_cards_q
                .get_single_mut()
                .unwrap()
                .make_active(&entity, &card);
        }
        draw_player_ev.send(DrawPlayer);
    }
}

pub fn play_card(
    mut active_cards_q: Query<&mut ActiveCards>,
    mut client: Client,
    mut draw_player_ev: EventWriter<DrawPlayer>,
) {
    info!("Play Card!");
    let mut active_cards_map = active_cards_q.get_single_mut().unwrap();

    let mut cards = active_cards_map.to_string();
    active_cards_map.clear();

    client.send_message::<PlayerActionChannel, PlayCard>(&PlayCard(cards));

    draw_player_ev.send(DrawPlayer);
    info!("Sended Cards");
}

pub fn spawn_player(
    mut commands: Commands,
    hand_q: Query<&ServerHand, With<LocalPlayer>>,
    // player_q: Query<Entity, With<LocalPlayer>>,
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

    // let player_entity = player_q.get_single().unwrap();

    for card_str in sl {
        let card_rs = Card::from_str(card_str);

        if let Ok(card) = card_rs {
            let card_entity = commands.spawn(card).id();
            global.player_cards.insert(card_entity, card);
            // commands.entity(player_entity).add_child(card_entity);
        } else {
            info!("SPAWN CARD ERROR: {}", card_str);
        }
    }

    draw_player_ev.send(DrawPlayer);
}
