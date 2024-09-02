use crate::card::CardNames;
use crate::user::User;
use crate::deck::Deck;

pub struct Game {
    pub(crate) user: User,
    pub(crate) dealer: User,
    pub(crate) deck: Deck,
}

impl Game {
    pub fn create_game() -> Game {
        let mut deck = Deck::deck_builder(); // Build the deck
        Deck::deck_shuffler(&mut deck);      // Shuffle the deck in place

        let hand = Deck::deck_dealer(&mut deck); // Deal a hand from the shuffled deck
        let split_hand = Vec::new();
        let dealer_split_hand = Vec::new();
        let dealer_hand = Deck::deck_dealer(&mut deck);

        // Pass a hand to the user and dealer
        let mut user = User::create_user(hand, split_hand);
        let dealer = User::create_dealer(dealer_hand, dealer_split_hand);

        let decklength = &deck.deck.len();
        println!("Remaining Cards in deck: {}", &decklength);

        Game { user, dealer, deck }
    }

    //win condition logic
    pub fn check_wincon(user: &mut User, dealer: &mut User) {
        let mut hand_value = 0;
        let mut dealer_hand_value = 0;
        let mut ace_count = 0;
        let mut dealer_ace_count = 0;
        for x in &user.hand {
            if let CardNames::Ace = x.card_name { //check if there's an ace
                ace_count += 1;
            }
            hand_value = hand_value + x.card_value;
        }
        if hand_value > 21 && ace_count > 1 { //if over 21, let ace have value 1 instead of 11
            hand_value = hand_value - 10;
        } else if hand_value > 21 {
            println!("Bust! you lose! Total: {}", &hand_value);
            std::process::exit(0) //You blow up the casino
        } else {
            println!("Your total is: {}", &hand_value);
        }

        for y in &dealer.hand {
            if let CardNames::Ace = y.card_name { //check if there's an ace
                dealer_ace_count += 1;
            }
            dealer_hand_value = dealer_hand_value + y.card_value;
        }
        if hand_value > 21 && ace_count > 1 { //if over 21, let ace have value 1 instead of 11
            hand_value = hand_value - 10;
        }
        else if dealer_hand_value > 21 {
            println!("Dealer Bust! You Win! {}", &dealer_hand_value);
            std::process::exit(0) //Dealer blows up the casino
        } else {
            println!("Dealer total is: {}", &dealer_hand_value);
        }

        if dealer_hand_value > hand_value {
            println!("Dealer wins! You lose!");
            std::process::exit(0)
        } else {
            println!("You win!");
            std::process::exit(0)
        }
    }

    //check if dealer needs to hit
    pub fn check_dealer(deck: &mut Deck, dealer: &mut User) {
        let mut dealer_hand_value = 0;
        for y in &dealer.hand {
            dealer_hand_value = dealer_hand_value + y.card_value;
        }
        if (dealer_hand_value < 16) {
            let mut hand = &mut dealer.hand;
            Deck::dealer_hit(deck, hand);
        }
    }

    pub fn end_game() {}
}
