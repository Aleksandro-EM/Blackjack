use crate::card::{Card, CardNames, CardSuits};
use rand::prelude::SliceRandom;
use rand::prelude::*;
use std;
use std::io;

pub struct Deck {
    pub(crate) deck: Vec<Card>,
}

impl Deck {
    pub fn deck_builder() -> Deck {
        let mut deckvector = Vec::new();
        for suit in 1..=4 {
            for name in 1..=13 {
                let card1 = Card::create_card(CardNames::to_name(name), CardSuits::to_suit(suit), CardNames::to_name(name).to_number());
                deckvector.push(card1);
            }
        }
        Deck { deck: deckvector }
    }

    pub fn deck_shuffler(deck: &mut Deck) {
        let mut rng = thread_rng();
        deck.deck.shuffle(&mut rng); // Shuffle the deck in place
        println!("Deck has been shuffled");
    }

    //Deal cards
    pub fn deck_dealer(deck: &mut Deck) -> Vec<Card> {
        let mut hand = Vec::new();

        if let Some(card) = deck.deck.pop() {
            hand.push(card);
        }

        if let Some(card2) = deck.deck.pop() {
            hand.push(card2);
        }
        hand
    }

    pub fn hit(deck: &mut Deck, hand: &mut Vec<Card>) {
        println!("input 'h' to hit, otherwise stay");
        let mut hit = String::new();
        io::stdin().read_line(&mut hit).expect("Failed to read name");
        if (hit.trim() == "h") {
            if let Some(card) = deck.deck.pop() {
                hand.push(card);
                println!("You hit!");
                for card in hand {
                    card.print_card();
                }
            } else {
                println!("The deck is out of cards!");
            }
        } else {
            println!("You decide to stay.")
        }
    }

    pub fn dealer_hit (deck: &mut Deck, hand: &mut Vec<Card>) {
        if let Some(card) = deck.deck.pop() {
            hand.push(card);
            println!("Dealer hits!");
            for card in hand {
                card.print_card();
            }
        } else {
            println!("The deck is out of cards!");
        }
    }
}
