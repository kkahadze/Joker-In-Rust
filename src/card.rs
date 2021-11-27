use std::fmt;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = match self {
            Suit::Diamonds => "♦",
            Suit::Clubs => "♣",
            Suit::Hearts => "♥",
            Suit::Spades => "♠",
        };
        write!(f, "{}", icon)
    }
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Colour {
    Red,
    Black,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub enum Rank {
    Ace,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
    Five,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display = match *self {
            Rank::Ace => "A",
            Rank::Joker => "JKR",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Five => "",
        };
        write!(f, "{}", display)
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
    pub colour: Colour,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        let colour = match suit {
            Suit::Diamonds | Suit::Hearts => Colour::Red,
            _ => Colour::Black,
        };
        Card { suit, rank, colour }
    }

    pub fn copy(&self) -> Card {
        Card::new(self.rank, self.suit)
    }

    pub fn is_joker(&self) -> bool {
        matches!(self.rank, Rank::Joker)
    }

    pub fn is_suit(&self, suit_in: Suit) -> bool {
        if self.is_joker() {
            false
        } else {
            self.suit == suit_in
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card;
        if self.rank == Rank::Joker {
            card = format!("{}", self.rank);
        } else if self.rank == Rank::Five {
            card = format!("{}", self.suit);
        } else {
            card = format!("{}{}", self.suit, self.rank);
        }
        write!(f, "{}", card)
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank && self.suit == other.suit
    }
}
