use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result as FmtResut};
use std::ops::AddAssign;

use bevy_ecs::prelude::Component;
use log::info;
use naia_bevy_shared::Property;

use super::card::Card;
use super::cards::Cards;

#[derive(Clone, Component, PartialEq, Eq, Default)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> FmtResut {
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

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cards.partial_cmp(&other.cards)
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.highest_value().cmp(other.highest_value())
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

impl From<String> for Hand {
    fn from(cards_str: String) -> Hand {
        if cards_str.is_empty() {
            return Hand::new();
        }

        let cards = cards_str
            .split(',')
            .map(|c_str| Card::from_str(c_str).unwrap())
            .collect::<Vec<Card>>();

        Hand { cards }
    }
}

impl From<Property<String>> for Hand {
    fn from(cards_str: Property<String>) -> Hand {
        if cards_str.is_empty() {
            return Hand::new();
        }

        let cards = cards_str
            .split(',')
            .map(|c_str| Card::from_str(c_str).unwrap())
            .collect::<Vec<Card>>();

        Hand { cards }
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
        }
    }

    pub fn from_str(cards_str: &str) -> Hand {
        let cards = cards_str
            .split(",")
            .map(|c_str| {
                Card::from_str(c_str).unwrap_or_else(|_| {
                    panic!("Not a known card {}", c_str);
                })
            })
            .collect::<Vec<Card>>();

        Hand { cards }
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
        Hand { cards }
    }

    #[allow(clippy::inherent_to_string_shadow_display)]
    pub fn to_string(&mut self) -> String {
        self.cards
            .iter()
            .map(|c| c.to_str())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
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

    pub fn total_value(&self) -> usize {
        self.cards.iter().map(|c| c.ordinal()).sum()
    }

    /// Returns the highest value card of the hand
    pub fn highest_value(&self) -> &Card {
        self.cards.iter().max().unwrap()
    }

    /// Clears the `Hand` (makes it empty)
    pub fn clear(&mut self) {
        self.cards.clear();
    }

    pub fn smallest_card(&mut self) -> Card {
        self.sort();
        *self.cards.get(0).unwrap()
    }

    pub fn remove_smallest_card(&mut self) -> Card {
        self.sort();
        self.cards.remove(0)
    }

    /// Removes a `Card` from the `Hand` and returns it, panics if index does not exist
    pub fn remove(&mut self, index: usize) -> Card {
        self.cards.remove(index)
    }

    /// Removes the first instance of every matching card from the `Hand`
    pub fn remove_cards(&mut self, cards: &[Card]) {
        for c in cards {
            info!("removing cards: {:?}", c.to_str());
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

    pub fn contain_3_c(&self) -> bool {
        let three_spade = Card::make_3_c();
        !self
            .cards
            .iter()
            .all(|card| three_spade.cmp(card) != Ordering::Equal)
    }

    pub fn sort(&mut self) {
        self.sort_suit_ascending_rank();
    }

    pub fn check_combination(&self) -> bool {
        self.is_in_combination()
    }
}
