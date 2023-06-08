use bevy::prelude::*;
use bevy_ggrs::RollbackIdProvider;
use rand::{seq::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::{card::Card, states::MainState};

#[derive(Component, Debug, Serialize, Deserialize, Clone, Reflect, Default)]
pub struct Deck {
    pub cards: Vec<Entity>,
    dealt_cards: Vec<Entity>,
}

pub struct DeckPlugin;

impl Plugin for DeckPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_deck.in_schedule(OnEnter(MainState::Game)));
    }
}

impl Deck {
    pub fn deal_one(&mut self) -> Result<Entity, &'static str> {
        if let Some(card) = self.cards.pop() {
            self.dealt_cards.push(card);
            Ok(card)
        } else {
            Err("No cards left")
        }
    }

    pub fn deal_card(&mut self, num_cards: usize) -> Vec<Entity> {
        let mut result: Vec<Entity> = Vec::with_capacity(num_cards);

        for _ in 0..13 {
            if let Ok(card) = self.deal_one() {
                result.push(card);
            } else {
                // No cards so no point continuing
                break;
            }
        }
        result
    }

    pub fn deal_thirteen(&mut self) -> Vec<Entity> {
        self.deal_card(13)
    }
}

#[derive(Component, Clone, Copy, Debug, Reflect, Default)]
pub struct ActiveCard(pub bool);

#[derive(Component, Reflect, Default)]
pub struct DealCard(pub bool);

#[derive(Component, Reflect, Default)]
pub struct Shuffle(pub bool);

fn spawn_deck(mut commands: Commands, mut rip: ResMut<RollbackIdProvider>) {
    let mut all_cards = Card::all_cards().to_vec();

    // let mut rng = thread_rng();
    //
    // all_cards.shuffle(&mut rng);

    let mut cards: Vec<Entity> = vec![];

    for card in all_cards {
        let card_id = commands.spawn((rip.next(), card, ActiveCard(false))).id();
        cards.push(card_id);
    }

    let deck = Deck {
        cards,
        dealt_cards: vec![],
    };

    info!("deck: {:?}", deck.cards);

    commands.spawn((rip.next(), deck, DealCard(false), Shuffle(false)));
}
