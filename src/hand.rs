use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
use std::ops::AddAssign;

use bevy::prelude::*;

use crate::card::Card;
use crate::cards::Cards;

#[derive(Clone, Debug, Component, PartialEq, Eq, PartialOrd)]
pub struct Hand {
    pub cards: Vec<Card>,
    pub picking_cards: Option<Vec<Entity>>,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let f_hand_combination = self.get_combination();
        let s_hand_combination = other.get_combination();

        println!("f {:?}, s {:?}", f_hand_combination, s_hand_combination);

        // TODO: this is wrong behavior
        if f_hand_combination != s_hand_combination {
            println!("They are not the same combination");
            return Ordering::Less;
        }

        let f_top_card = self.top_card();
        let s_top_card = other.top_card();

        match f_top_card {
            Some(f_card) => {
                if let Some(s_card) = s_top_card {
                    return f_card.cmp(&s_card);
                } else {
                    println!("S_card is NONE");
                    return Ordering::Less;
                }
            }
            None => {
                println!("F_card is NONE");
                return Ordering::Less;
            }
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut result = String::with_capacity(self.cards.len() * 3);
        self.cards.iter().enumerate().for_each(|(i, card)| {
            result.push_str(&card.to_str());
            if i < self.cards.len() - 1 {
                result.push(',');
            }
        });
        write!(f, "{}", result)
    }
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            cards: Vec::new(),
            picking_cards: None,
        }
    }
}

impl<'a> AddAssign<&'a Hand> for Hand {
    fn add_assign(&mut self, rhs: &Hand) {
        self.push_hand(rhs);
    }
}

impl AddAssign<Card> for Hand {
    fn add_assign(&mut self, rhs: Card) {
        self.push_card(rhs);
    }
}

impl Cards for Hand {
    fn cards(&self) -> &[Card] {
        self.cards.as_slice()
    }

    fn mut_cards(&mut self) -> &mut [Card] {
        self.cards.as_mut_slice()
    }
}

impl Hand {
    /// Create an empty hand
    pub fn new() -> Self {
        Self::default()
    }

    /// Makes a `Hand` from an existing hand
    pub fn from_hand(hand: &Hand) -> Hand {
        Hand::from_cards(hand.cards())
    }

    /// Makes a `Hand` from a slice
    pub fn from_cards(cards: &[Card]) -> Hand {
        Hand {
            cards: Vec::from(cards),
            picking_cards: None,
        }
    }

    /// Get highest card from a slice
    pub fn top_card(&self) -> Option<&Card> {
        let rs = self.cards().iter().max();
        rs
    }

    /// Constructs a `Hand` from a slice of strings with abbreviated card rank / suit values
    pub fn from_strings(card_slice: &[&str]) -> Hand {
        let cards = card_slice
            .iter()
            .map(|s| {
                Card::from_str(s).unwrap_or_else(|_| {
                    panic!("Not a known card {}", s);
                })
            })
            .collect::<Vec<Card>>();
        Hand {
            cards,
            picking_cards: None,
        }
    }

    /// Adds one `Card` to the `Hand`
    pub fn push_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    /// Adds zero or more cards to the `Hand`
    pub fn push_cards(&mut self, cards: &[Card]) {
        self.cards.extend(cards);
    }

    /// Adds zero or more cards from some other `Hand`
    pub fn push_hand(&mut self, other: &Hand) {
        self.cards.extend(other.cards());
    }

    /// Returns the number of cards
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Clears the `Hand` (makes it empty)
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    /// Removes a `Card` from the `Hand` and returns it, panics if index does not exist
    pub fn remove(&mut self, index: usize) -> Card {
        self.cards.remove(index)
    }

    /// Removes the first instance of every matching card from the `Hand`
    pub fn remove_cards(&mut self, cards: &[Card]) {
        for c in cards {
            let _ = self.remove_card(c);
        }
    }

    /// Removes the every instance of every matching card from the `Hand`
    pub fn remove_all_cards(&mut self, cards: &[Card]) {
        for c in cards {
            while self.remove_card(c) {}
        }
    }

    /// Removes first instance of the matching card from the `Hand`
    pub fn remove_card(&mut self, card: &Card) -> bool {
        if let Some(pos) = self.cards.iter().position(|c| c == card) {
            let _ = self.cards.remove(pos);
            true
        } else {
            false
        }
    }
}
