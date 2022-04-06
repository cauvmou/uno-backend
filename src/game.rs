
use crate::card::Card;

pub struct Player {
    id: u8,
    username: String,
    inventory: Vec<Card>,
}

pub struct GameInstance {
    players: Vec<Player>,
    unused_cards: Vec<Card>
}