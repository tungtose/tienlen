use std::cmp::Ordering;

use bevy::prelude::*;
use bevy_ggrs::{GGRSSchedule, PlayerInputs, RollbackIdProvider};
use rand::{rngs::mock::StepRng, seq::SliceRandom, thread_rng};

use crate::{
    card::Card,
    cards::Cards,
    deck::{ActiveCard, DealCard, Deck, Shuffle},
    hand::Hand,
    states::MainState,
    ui::{table::TableCards, ReloadUiEvent},
    GGRSConfig,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerEvent>()
            .add_event::<TurnUpdateEvent>()
            .add_event::<SpawnPlayerEvent>()
            .add_system(spawn_player.run_if(on_event::<SpawnPlayerEvent>()))
            .add_system(
                select_card.run_if(on_event::<PlayerEvent>()), // .before(play_card),
            )
            .add_systems(
                (
                    shuffle_deck.before(deal_card_to_player),
                    deal_card_to_player.after(shuffle_deck),
                )
                    .in_schedule(GGRSSchedule),
            );
        // .add_system(skip_turn.run_if(on_event::<PlayerEvent>()))
        // .add_system(play_card.run_if(on_event::<PlayerEvent>().and_then(playable)));
    }
}

// fn playable(
//     hand_query: Query<&PlayerHand, With<ActivePlayer>>,
//     card_query: Query<&Card>,
//     table_card_query: Query<&TableCards>,
// ) -> bool {
//     let hand = hand_query.get_single().unwrap();
//
//     let table = table_card_query.get_single().unwrap();
//
//     let play_cards: Vec<Card> = hand
//         .active_cards
//         .iter()
//         .map(|e| *card_query.get(*e).unwrap())
//         .collect();
//
//     let table_cards: Vec<Card> = table
//         .cards
//         .iter()
//         .map(|e| *card_query.get(*e).unwrap())
//         .collect();
//
//     let play_hand = Hand {
//         cards: play_cards.clone(),
//         picking_cards: None,
//     };
//
//     if !play_hand.is_in_combination() {
//         return false;
//     }
//
//     if table_cards.is_empty() {
//         return true;
//     }
//
//     if play_cards.len() != table_cards.len() {
//         println!("Two sequences are not the same length!");
//         return false;
//     }
//
//     let table_hand = Hand {
//         cards: table_cards,
//         picking_cards: None,
//     };
//
//     let ord_res = play_hand.cmp(&table_hand);
//
//     if let Ordering::Greater = ord_res {
//         return true;
//     }
//
//     false
// }
//
const TOTAL_PLAYER: usize = 2;
//
pub enum PlayerEventKind {
    SelectCard(Entity, PlayerPosition),
    Play,
    Skip,
}

pub struct PlayerEvent(pub PlayerEventKind);

pub struct SpawnPlayerEvent;

pub struct TurnUpdateEvent;
//
// pub fn skip_turn(
//     mut ev_player: EventReader<PlayerEvent>,
//     mut ev_playaction: EventWriter<TurnUpdateEvent>,
// ) {
//     for ev in ev_player.iter() {
//         if let PlayerEvent(PlayerEventKind::Skip) = ev {
//             ev_playaction.send(TurnUpdateEvent);
//         }
//     }
// }
//
// pub fn play_card(
//     mut ev_player: EventReader<PlayerEvent>,
//     mut ev_ui: EventWriter<ReloadUiEvent>,
//     mut ev_playaction: EventWriter<TurnUpdateEvent>,
//     mut hand_query: Query<&mut PlayerHand, With<ActivePlayer>>,
//     mut table_card_query: Query<&mut TableCards>,
//     card_query: Query<&Card>,
// ) {
//     let mut table_cards = table_card_query.get_single_mut().unwrap();
//     for ev in ev_player.iter() {
//         if let PlayerEvent(PlayerEventKind::Play) = ev {
//             let mut active_hand = hand_query.get_single_mut().unwrap();
//
//             if active_hand.active_cards.is_empty() {
//                 break;
//             }
//
//             // Store the previous cards to compare in the next turn (is it necessaries ?)
//             if !table_cards.cards.is_empty() {
//                 table_cards.last_cards = table_cards.cards.clone();
//             }
//
//             table_cards.cards = active_hand.to_table(&card_query);
//
//             for card in active_hand.active_cards.clone() {
//                 active_hand.cards.retain(|card_e| *card_e != card);
//             }
//
//             active_hand.active_cards = vec![];
//
//             ev_ui.send(ReloadUiEvent);
//             ev_playaction.send(TurnUpdateEvent);
//         }
//     }
// }
//
pub fn select_card(
    mut ev_player: EventReader<PlayerEvent>,
    mut ev_ui: EventWriter<ReloadUiEvent>,
    mut card_query: Query<(&mut ActiveCard, &Card)>,
    mut hand_query: Query<(&mut PlayerHand, &PlayerPosition), With<ActivePlayer>>,
) {
    for ev in ev_player.iter() {
        match ev {
            PlayerEvent(PlayerEventKind::SelectCard(entity, p_pos)) => {
                let (mut active_card, card) = card_query.get_mut(*entity).unwrap();

                println!("card ordinal {:?}", card.ordinal());

                for (mut p_hand, pos) in hand_query.iter_mut() {
                    if p_pos.0 == pos.0 {
                        let matched_card = p_hand.active_cards.contains(entity);
                        if matched_card {
                            p_hand
                                .active_cards
                                .retain(|card_entity| card_entity != entity);
                        } else {
                            p_hand.active_cards.push(*entity);
                        }

                        active_card.0 = !active_card.0;
                    }
                }
                ev_ui.send(ReloadUiEvent);
            }
            _ => {}
        }
    }
}
//
#[derive(Component)]
pub struct Player {
    pub handle: usize,
}

#[derive(Component, Reflect, Default)]
pub struct ActivePlayer;

#[derive(Component, Debug, Default, Reflect)]
pub struct PlayerHand {
    pub cards: Vec<Entity>,
    pub active_cards: Vec<Entity>,
}

impl PlayerHand {
    pub fn new(cards: Vec<Entity>, card_query: &Query<&Card>) -> Self {
        Self {
            cards: vec![],
            active_cards: vec![],
        }
        // let mut hand: Vec<(&Entity, &Card)> = cards
        //     .iter()
        //     .map(|card_id| {
        //         let card = card_query.get(*card_id).unwrap();
        //         (card_id, card)
        //     })
        //     .collect();
        //
        // hand.sort_by(|a, b| a.1.cmp_rank_suit(b.1));
        //
        // let cards_r: Vec<Entity> = hand.iter().map(|(k, _v)| **k).collect();
        //
        // Self {
        //     cards: cards_r,
        //     active_cards: vec![],
        // }
    }

    pub fn sort(&self, card_query: &Query<&Card>) -> Self {
        let mut hand: Vec<(&Entity, &Card)> = self
            .cards
            .iter()
            .map(|card_id| {
                let card = card_query.get(*card_id).unwrap();
                (card_id, card)
            })
            .collect();

        hand.sort_by(|a, b| a.1.cmp_rank_suit(b.1));

        let cards_r: Vec<Entity> = hand.iter().map(|(k, _v)| **k).collect();

        Self {
            cards: cards_r,
            active_cards: vec![],
        }
    }

    pub fn to_table(&self, card_query: &Query<&Card>) -> Vec<Entity> {
        let mut hand: Vec<(&Entity, &Card)> = self
            .active_cards
            .iter()
            .map(|card_id| {
                let card = card_query.get(*card_id).unwrap();
                (card_id, card)
            })
            .collect();

        hand.sort_by(|a, b| a.1.cmp_rank_suit(b.1));

        let cards_r: Vec<Entity> = hand.iter().map(|(k, _v)| **k).collect();

        cards_r
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Reflect, Default)]
pub struct PlayerPosition(pub usize);

pub fn spawn_player(
    mut commands: Commands,
    mut deck_q: Query<&mut Deck>,
    // mut ev_ui: EventWriter<ReloadUiEvent>,
    card_query: Query<&Card>,
    mut rip: ResMut<RollbackIdProvider>,
) {
    let mut deck = deck_q.get_single_mut().unwrap();
    for player_num in 0..TOTAL_PLAYER {
        let player_hand = PlayerHand::new(deck.deal_thirteen(), &card_query);
        let player_pos = PlayerPosition(player_num);

        if player_num == 0 {
            commands.spawn((
                Player { handle: player_num },
                rip.next(),
                player_hand,
                player_pos,
                ActivePlayer,
            ));
        } else {
            commands.spawn((
                Player { handle: player_num },
                rip.next(),
                player_hand,
                player_pos,
            ));
        }
    }
    // info!("Start event draw UI");
    // ev_ui.send(ReloadUiEvent);
}

fn shuffle_deck(mut deck_q: Query<(&mut Deck, &mut Shuffle)>) {
    let (mut deck, mut shuffle) = deck_q.get_single_mut().unwrap();
    if !shuffle.0 {
        info!("SHUFFLING");
        let mut rng = StepRng::new(2, 4);
        deck.cards.shuffle(&mut rng);

        info!("deck after: {:?}", deck.cards);
        shuffle.0 = true;
    }
}

fn deal_card_to_player(
    mut player_q: Query<(&mut PlayerHand, &Player)>,
    mut deck_q: Query<(&mut Deck, &mut DealCard, &Shuffle)>,
    mut ev_ui: EventWriter<ReloadUiEvent>,
) {
    let (mut deck, mut deal_card, shuffle) = deck_q.get_single_mut().unwrap();
    if !deal_card.0 && shuffle.0 {
        info!("DEAL CARD!");
        for (mut hand, _player) in player_q.iter_mut() {
            hand.cards = deck.deal_thirteen();
            info!("cur hand: {:?}", hand.cards);
        }
        deal_card.0 = true;
        ev_ui.send(ReloadUiEvent);
    }
}
