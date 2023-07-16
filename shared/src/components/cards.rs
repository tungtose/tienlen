use rand;
use std::cmp::Ordering;

use crate::components::hand::Hand;

use super::{card::Card, rank::Rank, suit::Suit};
use rand::{prelude::SliceRandom, thread_rng};

#[test]
fn test_shuffle() {
    // This code is going create a bunch of decks and shuffle them. It will test that the cards at ends of the deck appear to be shuffled.
    // let loop_count = 50;
    // let mut top_matches = 0;
    // let mut bottom_matches = 0;
    //
    // for _ in 0..loop_count {
    //     let mut d = Deck::new();
    //     // Get cards before shuffling
    //     let t1 = d.top_card().unwrap();
    //     let b1 = d.bottom_card().unwrap();
    //     // Shuffle
    //     d.shuffle();
    //     // Get end cards after shuffling
    //     let t2 = d.top_card().unwrap();
    //     let b2 = d.bottom_card().unwrap();
    //     // Increment if top and bottom appear to be unshuffled
    //     if t1 == t2 {
    //         top_matches += 1;
    //     }
    //     if b1 == b2 {
    //         bottom_matches += 1;
    //     }
    // }
    //
    // println!(
    //     "top card matched {} times, bottom card matched {} times",
    //     top_matches, bottom_matches
    // );
    //
    // // We expect shuffling o both top and bottom at least some of the iterations of the loop
    // assert!(top_matches < loop_count);
    // assert!(bottom_matches < loop_count);
}

#[test]
fn test_evaluate_combination() {
    let double_sequences = Hand::from_strings(&["4D", "5H", "5D", "6C", "6S", "4S"]);
    let non_double_sequences = Hand::from_strings(&["4D", "4S", "7H", "5D", "6C", "6S"]);
    let sequence = Hand::from_strings(&["4D", "6C", "5S", "7S"]);
    let sequence2 = Hand::from_strings(&["KS", "AS", "2S"]);
    let triple = Hand::from_strings(&["4D", "4S", "4H"]);
    let pair = Hand::from_strings(&["5S", "5H"]);
    let single = Hand::from_strings(&["5S"]);
    let quartet = Hand::from_strings(&["5S", "5H", "5D", "5C"]);
    let nonsense = Hand::from_strings(&["5S", "5H", "7H"]);

    assert_eq!(
        evaluate_combination(double_sequences.cards()),
        ThirteenCombination::DoubleSequence
    );
    assert_eq!(
        evaluate_combination(non_double_sequences.cards()),
        ThirteenCombination::NonSense
    );
    assert_eq!(
        evaluate_combination(sequence.cards()),
        ThirteenCombination::Sequence
    );
    assert_eq!(
        evaluate_combination(triple.cards()),
        ThirteenCombination::Triple
    );
    assert_eq!(
        evaluate_combination(pair.cards()),
        ThirteenCombination::Pair
    );
    assert_eq!(
        evaluate_combination(nonsense.cards()),
        ThirteenCombination::NonSense
    );
    assert_eq!(
        evaluate_combination(single.cards()),
        ThirteenCombination::Single
    );
    assert_eq!(
        evaluate_combination(quartet.cards()),
        ThirteenCombination::Quartet
    );
    assert_eq!(
        evaluate_combination(sequence2.cards()),
        ThirteenCombination::NonSense
    );
}

#[test]
fn test_is_double_sequence() {
    let double_sequences = Hand::from_strings(&["4D", "5H", "5D", "6C", "6S", "4S"]);
    let non_double_sequences = Hand::from_strings(&["4D", "4S", "7H", "5D", "6C", "6S"]);

    assert!(is_double_sequences(double_sequences.cards()));
    assert!(!is_double_sequences(non_double_sequences.cards()));
}

#[test]
fn test_is_sequence() {
    // let seq_hand = Hand::from_strings(&["5D", "4D", "3D", "6C"]);
    let seq_hand = Hand::from_strings(&["8D", "9H", "4D", "5D", "6C", "7S"]);
    let non_seq_hand = Hand::from_strings(&["3D", "9D", "5D"]);

    assert!(is_sequences(seq_hand.cards()));
    assert!(!is_sequences(non_seq_hand.cards()));
}

/// Sorts the slice by suit then rank (low to high)
fn sort_suit_ascending_rank(cards: &mut [Card]) {
    cards.sort_by(|a, b| a.cmp_rank_suit(b));
}

/// Sorts the slice by rank(high to low) and then suit
fn sort_suit_descending_rank(_cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    // cards.sort_by(|a, b| a.cmp_suit_then_desc_rank(b));
    todo!()
}

/// Sorts the slice by rank(high to low) and then suit
fn sort_descending_rank_suit(_cards: &mut [Card]) {
    // Reverse sort (since default is low to high)
    todo!()
    // cards.sort_by(|a, b| a.cmp_desc_rank_then_suit(b));
}

/// Returns cards of the specified rank
pub fn cards_of_rank(cards: &[Card], rank: Rank) -> Vec<Card> {
    cards.iter().filter(|c| c.rank == rank).cloned().collect()
}

/// Returns cards of the specified suit
pub fn cards_of_suit(cards: &[Card], suit: Suit) -> Vec<Card> {
    cards.iter().filter(|c| c.suit == suit).cloned().collect()
}

/// Shuffles the slice of cards
pub fn shuffle(cards: &mut [Card]) {
    let mut rng = thread_rng();

    cards.shuffle(&mut rng)
}

#[derive(Debug, PartialEq, Eq)]
pub enum ThirteenCombination {
    Single,
    Pair,
    Triple,
    Quartet,
    Sequence,
    DoubleSequence,
    NonSense,
}

pub fn get_ords_rank(cards: &[Card]) -> Vec<usize> {
    let mut ords: Vec<usize> = cards.iter().map(|card| card.rank.ordinal()).collect();

    ords.sort_by(|a, b| a.cmp(b));

    ords
}

pub fn balance_array(ords: Vec<usize>) -> bool {
    let first = ords.first().unwrap_or(&0);

    ords.iter().all(|x| first.cmp(x) == Ordering::Equal)
}

pub fn all_cards_the_same_rank(cards: &[Card]) -> bool {
    let ords = get_ords_rank(cards);

    balance_array(ords)
}

pub fn is_double_sequences(cards: &[Card]) -> bool {
    let ords = get_ords_rank(cards);

    let is_three_pair = ords
        .chunks(2)
        .all(|chunks_cards| balance_array(chunks_cards.to_vec()));

    let half_ords: Vec<usize> = ords
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| *v)
        .collect();

    if is_three_pair && is_number_sequence(half_ords) {
        return true;
    }

    false
}

pub fn is_sequences(cards: &[Card]) -> bool {
    let contain_rank_two = cards.iter().all(|c| c.rank != Rank::Two);

    if !contain_rank_two {
        return false;
    }

    let mut ords: Vec<usize> = cards.iter().map(|card| card.rank.ordinal()).collect();

    ords.sort_by(|a, b| a.cmp(b));

    let is_sq = ords.iter().enumerate().skip(1).all(|(i, x)| {
        let prev = ords.get(i - 1).unwrap_or(&0);
        x - prev == 1
    });

    is_sq
}

pub fn is_number_sequence(ords: Vec<usize>) -> bool {
    ords.iter().enumerate().skip(1).all(|(i, x)| {
        let prev = ords.get(i - 1).unwrap_or(&0);
        x - prev == 1
    })
}

pub fn evaluate_combination(cards: &[Card]) -> ThirteenCombination {
    match cards.len() {
        1 => {
            return ThirteenCombination::Single;
        }
        2 => {
            if all_cards_the_same_rank(cards) {
                return ThirteenCombination::Pair;
            }

            ThirteenCombination::NonSense
        }
        3 => {
            if all_cards_the_same_rank(cards) {
                return ThirteenCombination::Triple;
            }

            if is_sequences(cards) {
                return ThirteenCombination::Sequence;
            }

            ThirteenCombination::NonSense
        }
        4 => {
            if all_cards_the_same_rank(cards) {
                return ThirteenCombination::Quartet;
            }

            if is_sequences(cards) {
                return ThirteenCombination::Sequence;
            }

            ThirteenCombination::NonSense
        }
        6 => {
            if is_sequences(cards) {
                return ThirteenCombination::Sequence;
            }

            if is_double_sequences(cards) {
                return ThirteenCombination::DoubleSequence;
            }

            ThirteenCombination::NonSense
        }
        5 | 7 | 8 | 9 | 10 | 11 | 12 | 13 => {
            if is_sequences(cards) {
                return ThirteenCombination::Sequence;
            }
            ThirteenCombination::NonSense
        }
        _ => ThirteenCombination::NonSense,
    };

    ThirteenCombination::NonSense
}

/// Certain actions are common to a deck and a hand of cards
pub trait Cards {
    /// Return the cards as a slice
    fn cards(&self) -> &[Card];

    /// Return the cards as a mutable slice
    fn mut_cards(&mut self) -> &mut [Card];

    /// Shuffle the cards into a random order
    fn shuffle(&mut self) {
        shuffle(self.mut_cards());
    }

    fn get_combination(&self) -> ThirteenCombination {
        evaluate_combination(self.cards())
    }

    fn is_in_combination(&self) -> bool {
        let combo = evaluate_combination(self.cards());

        if let ThirteenCombination::NonSense = combo {
            return false;
        }

        true
    }

    /// Sort the cards by suit and then by rank (low to high)
    fn sort_suit_ascending_rank(&mut self) {
        sort_suit_ascending_rank(self.mut_cards());
    }

    /// Sorts the cards by suit and then by rank (high to low)
    fn sort_suit_descending_rank(&mut self) {
        sort_suit_descending_rank(self.mut_cards());
    }

    /// Sort the cards by rank (high to low) and then by suit
    fn sort_descending_rank_suit(&mut self) {
        sort_descending_rank_suit(self.mut_cards());
    }
}
