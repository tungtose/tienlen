use std::cmp::Ordering;
use std::str::FromStr;

use bevy_ecs::component::Component;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, Serialize, Deserialize, Component)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Default for Suit {
    fn default() -> Self {
        Self::Hearts
    }
}

impl FromStr for Suit {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "S" => Ok(Suit::Spades),
            "C" => Ok(Suit::Clubs),
            "D" => Ok(Suit::Diamonds),
            "H" => Ok(Suit::Hearts),
            _ => Err("Invalid suit".into()),
        }
    }
}

impl Ord for Suit {
    fn cmp(&self, other: &Suit) -> Ordering {
        let self_ord = self.ordinal();
        let other_ord = other.ordinal();
        if self_ord < other_ord {
            return Ordering::Less;
        }
        if self_ord > other_ord {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl Suit {
    pub fn iterator() -> std::slice::Iter<'static, Suit> {
        Suit::gen_suit().into_iter()
    }

    pub fn ordinal(&self) -> usize {
        match *self {
            Suit::Hearts => 3,
            Suit::Diamonds => 2,
            Suit::Clubs => 1,
            Suit::Spades => 0,
        }
    }

    pub fn from_char(ch: char) -> Result<Suit, &'static str> {
        match ch {
            'S' => Ok(Suit::Spades),
            'C' => Ok(Suit::Clubs),
            'D' => Ok(Suit::Diamonds),
            'H' => Ok(Suit::Hearts),
            _ => Err("Invalid suit"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Suit::Spades => 'S',
            Suit::Hearts => 'H',
            Suit::Diamonds => 'D',
            Suit::Clubs => 'C',
        }
    }

    pub fn get_asset_path(&self) -> String {
        match *self {
            Suit::Spades => "spade".to_string(),
            Suit::Hearts => "heart".to_string(),
            Suit::Diamonds => "diamond".to_string(),
            Suit::Clubs => "club".to_string(),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            Suit::Spades => "Spades",
            Suit::Hearts => "Hearts",
            Suit::Diamonds => "Diamonds",
            Suit::Clubs => "Clubs",
        }
    }

    pub fn gen_suit() -> &'static [Suit] {
        static SUITS: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Clubs, Suit::Spades];
        &SUITS
    }
}
