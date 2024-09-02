use std::io;
use crate::card::Card;

pub(crate) struct User {
    name: String,
    pub(crate) hand: Vec<Card>,
    split_hand: Vec<Card>

}

impl User {
    pub fn create_user(hand: Vec<Card>, split_hand: Vec<Card>) -> User {
        println!("Please input your name");

        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        println!("{} has been dealt the following hand:", name);
        for card in &hand {
            card.print_card();
        }
        User { name, hand, split_hand }
    }

    pub fn create_dealer(hand: Vec<Card>, dealer_split_hand: Vec<Card>) -> User {
        let name = String::from("Dealer");
        println!("{}  has been dealt the following hand:", &name);
        for card in &hand {
            card.print_card();
        }
        User {name, hand, split_hand: dealer_split_hand}
    }
}
