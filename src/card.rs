use std::fmt;
use crate::CardNumber;
pub trait Copy: Clone { }
#[derive(Debug)]
pub struct Card {
    card_suit: CardSuits,
    pub(crate) card_name: CardNames,
    pub(crate) card_value: CardNumber,
}

impl Card {
    pub(crate) fn clone(card: Card) -> Card {
        let card2 = card;
        card2
    }
}

impl Card {
    pub(crate) fn print_card(&self) -> String {
        let suit = &self.card_suit;
        let name = &self.card_name;
        let full_card_name = format!("{} of {}", name, suit);
        println!("{}",full_card_name);
        full_card_name
    }

    pub(crate) fn create_card(name: CardNames, suit: CardSuits, value: CardNumber) -> Card {
        Card {
            card_suit: suit,
            card_name: name,
            card_value: value,
        }
    }
}

#[derive(Debug)]
pub enum CardSuits {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

//Format CardSuits for print
impl std::fmt::Display for CardSuits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardSuits::Hearts => write!(f, "Hearts ♥"),
            CardSuits::Diamonds => write!(f, "Diamonds ♦"),
            CardSuits::Spades => write!(f, "Spades ♠"),
            CardSuits::Clubs => write!(f, "Clubs ♣"),
        }
    }
}
//match numbers to CardSuits for shuffle
impl CardSuits {
    pub(crate) fn to_suit(suit: CardNumber) -> Self {
        match suit {
            1 => CardSuits::Spades,
            2 => CardSuits::Hearts,
            3 => CardSuits::Diamonds,
            4 => CardSuits::Spades,
            _ => panic!("Vat is happening?!")
        }
    }
}

#[derive(Debug)]
pub enum CardNames {
    Ace,
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
}

//Assign numerical values to CardNames
impl CardNames {
    pub(crate) fn to_number(&self) -> CardNumber {
        match &self {
            CardNames::Ace => 11,
            CardNames::Two => 2,
            CardNames::Three => 3,
            CardNames::Four => 4,
            CardNames::Five => 5,
            CardNames::Six => 6,
            CardNames::Seven => 7,
            CardNames::Eight => 8,
            CardNames::Nine => 9,
            CardNames::Ten => 10,
            CardNames::Jack => 10,
            CardNames::Queen => 10,
            CardNames::King => 10,
        }
    }
    pub(crate) fn to_name(number: CardNumber) -> Self {
        match number {
            1 => CardNames::Ace,
            2 => CardNames::Two,
            3 => CardNames::Three,
            4 => CardNames::Four,
            5 => CardNames::Five,
            6 => CardNames::Six,
            7 => CardNames::Seven,
            8 => CardNames::Eight,
            9 => CardNames::Nine,
            10 => CardNames::Ten,
            11 => CardNames::Jack,
            12 => CardNames::Queen,
            13 => CardNames::King,
            _ => panic!("Number exceeded card names")
        }
    }
}
//Format CardNames for print
impl std::fmt::Display for CardNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardNames::Ace => write!(f, "Ace"),
            CardNames::Two => write!(f, "Two"),
            CardNames::Three => write!(f, "Three"),
            CardNames::Four => write!(f, "Four"),
            CardNames::Five => write!(f, "Five"),
            CardNames::Six => write!(f, "Six"),
            CardNames::Seven => write!(f, "Seven"),
            CardNames::Eight => write!(f, "Eight"),
            CardNames::Nine => write!(f, "Nine"),
            CardNames::Ten => write!(f, "Ten"),
            CardNames::Jack => write!(f, "Jack"),
            CardNames::Queen => write!(f, "Queen"),
            CardNames::King => write!(f, "King"),
        }
    }
}