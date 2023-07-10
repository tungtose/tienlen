use std::cmp::Ordering;
use std::slice::Iter;

use serde::{Deserialize, Serialize};

use Rank::*;

#[derive(Copy, Clone, Hash, Eq, PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[allow(clippy::derive_ord_xor_partial_ord)]
impl Ord for Rank {
    fn cmp(&self, other: &Rank) -> Ordering {
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

impl Default for Rank {
    fn default() -> Self {
        Rank::Ace
    }
}

impl Rank {
    pub fn iterator() -> Iter<'static, Rank> {
        Rank::ranks().into_iter()
    }

    pub fn ordinal(&self) -> usize {
        match *self {
            Three => 1,
            Four => 2,
            Five => 3,
            Six => 4,
            Seven => 5,
            Eight => 6,
            Nine => 7,
            Ten => 8,
            Jack => 9,
            Queen => 10,
            King => 11,
            Ace => 12,
            Two => 13,
        }
    }

    pub fn to_char(&self) -> char {
        match *self {
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
            Nine => '9',
            Ten => 'T',
            Jack => 'J',
            Queen => 'Q',
            King => 'K',
            Ace => 'A',
        }
    }

    pub fn from_char(ch: char) -> Result<Rank, &'static str> {
        let rank = match ch {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'T' => Ten,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' | '1' => Ace,
            _ => return Err("Invalid rank"),
        };
        Ok(rank)
    }

    pub fn to_str(&self) -> &'static str {
        match *self {
            Two => "Two",
            Three => "Three",
            Four => "Four",
            Five => "Five",
            Six => "Six",
            Seven => "Seven",
            Eight => "Eight",
            Nine => "Nine",
            Ten => "Ten",
            Jack => "Jack",
            Queen => "Queen",
            King => "King",
            Ace => "Ace",
        }
    }
    pub fn ranks() -> &'static [Rank] {
        static RANKS: [Rank; 13] = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];
        &RANKS[..]
    }
}
