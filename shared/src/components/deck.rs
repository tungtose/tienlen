use std::result::Result;
use std::vec::Vec;

use rand::{prelude::SliceRandom, thread_rng};
use serde::{Deserialize, Serialize};

use super::card::Card;
use super::cards::Cards;
use super::hand::Hand;

/// The `Deck` represents a deck of zero or more cards. A default deck is 52 playing cards.
/// Internally the deck consists of two stacks consisting of dealt and undealt cards. The dealt stack
/// receives cards as they are dealt from the undealt stack.
///
/// The deck may be `reset()` to return it to its original state. A deck may be `shuffle()`'d to randomize
/// its order. Shuffling uses a Knuth shuffle.
///
/// A deck can contain more than one card with the same rank / suit combination although by default
/// it does not.
///
/// A deck cannot have more cards added or removed to it once it is created.
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Deck {
    /// A deck contains zero or more cards
    cards: Vec<Card>,
    /// Dealt cards are cards which have been dealt in calls but are still members of the deck
    /// they remain dealt until the deck is reshuffled or reset.
    dealt_cards: Vec<Card>,
}

impl Cards for Deck {
    fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [Card] {
        self.cards.as_mut_slice()
    }

    fn shuffle(&mut self) {
        crate::components::cards::shuffle(self.mut_cards());
    }
}

impl Deck {
    /// Creates a new `Deck` containing the standard set of 52 cards
    pub fn new() -> Deck {
        Deck::from_cards(Card::all_cards())
    }

    /// Creates a new `Deck` containing the specified cards
    pub fn from_cards(cards: &[Card]) -> Deck {
        let mut rng = thread_rng();

        let mut mut_cards = cards.to_vec();

        mut_cards.shuffle(&mut rng);

        Deck {
            cards: mut_cards,
            dealt_cards: Vec::with_capacity(cards.len()),
        }
    }

    /// Returns the number of remaining undealt cards in the `Deck`
    pub fn undealt_count(&self) -> usize {
        self.cards.len()
    }

    /// Returns the number of dealt cards in the `Deck`
    pub fn dealt_count(&self) -> usize {
        self.dealt_cards.len()
    }

    /// Returns the number of cards, dealt or undealt, within the `Deck`
    pub fn count(&self) -> usize {
        self.undealt_count() + self.dealt_count()
    }

    /// Returns the collection of dealt cards
    pub fn dealt_cards(&self) -> &[Card] {
        self.dealt_cards.as_slice()
    }

    /// Tells you the top card (very next to be drawn) in the undealt deck
    /// without dealing it.
    pub fn top_card(&self) -> Option<Card> {
        self.cards().last().map(|card| *card)
    }

    /// Tells you the bottom card (very last to be drawn) in the undealt deck
    /// without dealing it.
    pub fn bottom_card(&self) -> Option<Card> {
        self.cards().first().map(|card| *card)
    }

    /// Deals the card from the undealt pile. If there are no cards left, the function
    /// will return an error.
    pub fn deal_one(&mut self) -> Result<Card, &'static str> {
        if let Some(card) = self.cards.pop() {
            self.dealt_cards.push(card);
            Ok(card)
        } else {
            Err("No cards left")
        }
    }

    /// Deals one or more card from the undealt pile and returns them as an array.
    pub fn deal(&mut self, numcards: usize) -> Vec<Card> {
        let mut result: Vec<Card> = Vec::with_capacity(numcards);
        for _ in 0..numcards {
            if let Ok(card) = self.deal_one() {
                result.push(card);
            } else {
                // No cards so no point continuing
                break;
            }
        }
        result
    }

    pub fn deal_str(&mut self, numcards: usize) -> String {
        self.deal(numcards)
            .iter()
            .map(|c| c.to_str())
            .collect::<String>()
    }

    /// Deals one or more card straight to the `Hand`. Returns the number of cards dealt.
    pub fn deal_to_hand(&mut self, hand: &mut Hand, numcards: usize) -> usize {
        let mut dealt: usize = 0;
        for _ in 0..numcards {
            if let Ok(card) = self.deal_one() {
                dealt += 1;
                hand.push_card(card);
            } else {
                // No cards so no point continuing
                break;
            }
        }
        dealt
    }

    /// Return the dealt cards back to the end of the undealt pile. Order is preserved according
    /// to the default order or the last shuffle.
    pub fn reset(&mut self) {
        // Put cards back into undealt deck in reverse order
        self.cards.extend(self.dealt_cards.iter().rev());
        self.dealt_cards.clear();
    }

    /// shuffles
    pub fn shuff(&mut self) {
        self.shuffle();
    }

    /// Resets and shuffles the deck
    pub fn reset_shuffle(&mut self) {
        self.reset();
        self.shuffle();
    }
}
