use std::cmp::Ordering;
use std::slice::Iter;

use bevy_ecs::prelude::Component;
use serde::{Deserialize, Serialize};

use super::{rank::Rank, suit::Suit};

#[derive(
    Component, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Debug, Serialize, Deserialize, Default,
)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        return self.cmp_rank_suit(other);
    }
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn cmp_rank(&self, other: &Card) -> Ordering {
        return self.rank.cmp(&other.rank);
    }

    pub fn make_3C() -> Card {
        Card {
            suit: Suit::Clubs,
            rank: Rank::Three,
        }
    }

    pub fn cmp_rank_suit(&self, other: &Card) -> Ordering {
        let cmp_rank_result: Ordering = self.rank.cmp(&other.rank);
        let cmp_suit_result: Ordering = self.suit.cmp(&other.suit);

        if cmp_rank_result == Ordering::Equal {
            return cmp_suit_result;
        }
        cmp_rank_result
    }

    pub fn from_str(s: &str) -> Result<Card, &'static str> {
        if s.len() != 2 {
            return Err("Card string must be length equal to 2");
        }

        let str = s.to_string();
        let mut char = str.chars();
        let char_rank = char.next().unwrap();
        let char_suit = char.next().unwrap();

        if let Ok(rank) = Rank::from_char(char_rank) {
            if let Ok(suit) = Suit::from_char(char_suit) {
                return Ok(Card::new(rank, suit));
            }
        }

        Err("Error parsing card: Invalid string")
    }

    pub fn to_str(&self) -> String {
        format!("{}{}", self.rank.to_char(), self.suit.to_char())
    }

    pub fn to_path(&self) -> String {
        let rank = self.rank;
        let suit = self.suit;

        let mut path = "cards/standard/solitaire/individuals".to_string();

        match suit {
            Suit::Hearts => path.push_str("/heart"),
            Suit::Diamonds => path.push_str("/diamond"),
            Suit::Clubs => path.push_str("/club"),
            Suit::Spades => path.push_str("/spade"),
        }

        match rank {
            Rank::Two => path.push_str("/2"),
            Rank::Three => path.push_str("/3"),
            Rank::Four => path.push_str("/4"),
            Rank::Five => path.push_str("/5"),
            Rank::Six => path.push_str("/6"),
            Rank::Seven => path.push_str("/7"),
            Rank::Eight => path.push_str("/8"),
            Rank::Nine => path.push_str("/9"),
            Rank::Ten => path.push_str("/10"),
            Rank::Jack => path.push_str("/11"),
            Rank::Queen => path.push_str("/12"),
            Rank::King => path.push_str("/13"),
            Rank::Ace => path.push_str("/1"),
        }

        path.push_str(".png");

        return path;
    }

    pub fn name(&self) -> String {
        format!("{} of {}", self.rank.to_str(), self.suit.to_str())
    }

    /// Returns an ordinal for the card which is a unique number which can be used for indexing
    pub fn ordinal(&self) -> usize {
        // self.suit.ordinal() * 13 + self.rank.ordinal()
        self.rank.ordinal() * 13 + self.suit.ordinal()
    }

    /// Tests if the card is Hearts
    pub fn is_hearts(&self) -> bool {
        self.suit == Suit::Hearts
    }

    /// Tests if the card is Clubs
    pub fn is_clubs(&self) -> bool {
        self.suit == Suit::Clubs
    }

    /// Tests if the card is Spades
    pub fn is_spades(&self) -> bool {
        self.suit == Suit::Spades
    }

    /// Tests if the card is Diamonds
    pub fn is_diamonds(&self) -> bool {
        self.suit == Suit::Diamonds
    }

    /// Returns an array slice containing all the cards in a standard 52-card deck
    pub fn all_cards() -> &'static [Card] {
        static CARDS: [Card; 52] = [
            Card {
                suit: Suit::Spades,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Three,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Four,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Five,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Nine,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Spades,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Three,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Four,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Five,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Nine,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Three,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Four,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Five,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Nine,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Two,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Three,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Four,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Five,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Six,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Seven,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Eight,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Nine,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ten,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Jack,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Queen,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::King,
            },
            Card {
                suit: Suit::Clubs,
                rank: Rank::Ace,
            },
        ];
        &CARDS
    }

    pub fn iterator() -> Iter<'static, Card> {
        Card::all_cards().into_iter()
    }
}
