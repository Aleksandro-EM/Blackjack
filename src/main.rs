use crate::game::Game;
use crate::deck::Deck;
mod card;
mod deck;
mod user;
mod game;

type CardNumber = u8;

fn main() {
    let mut new_game = Game::create_game();
    Deck::hit(&mut new_game.deck, &mut new_game.user.hand);
    Game::check_dealer(&mut new_game.deck, &mut new_game.dealer);
    Game::check_wincon(&mut new_game.user, &mut new_game.dealer);
}