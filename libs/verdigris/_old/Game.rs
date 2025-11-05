use core::error;
use std::collections::HashMap;

// use std::error::Error;
use thiserror::Error; // 1. Import the Error derive macro

pub struct Game{
    pub players: Vec<Player>
}

impl Game {
    fn is_two_player_game(&self) -> bool { self.players.len() == 2 }
    fn is_multiplayer_game(&self) -> bool { self.players.len() > 2 }

    
}

pub struct Player{
    pub deck: Deck
}

pub struct Deck{
    pub cards: Vec<Card>
}

#[derive(Debug, Error, PartialEq, Eq)] 
pub enum DeckValidationError {
    #[error("Deck must contain at least 60 cards, but found {0}")]
    LessThan60Cards(usize),

    #[error("Deck contains more than 4 copies of card: '{0}' (found {1})")]
    TooManyCopies(String, usize), // (Example of a 2nd error type)
}

impl Deck {
    fn is_valid_constructed_deck(&self) -> Result<(), Vec<DeckValidationError>> {
        let mut errors: Vec<DeckValidationError> = Vec::new();
        if self.cards.len() < 60 { 
            errors.push(DeckValidationError::LessThan60Cards(self.cards.len()));
        }
        let mut card_counts: HashMap<&Card, usize> = HashMap::new();
        for card in &self.cards {
            *card_counts.entry(card).or_insert(0) += 1;
        }
        let count_errors = card_counts.into_iter()
            .filter(|(_card, count)| *count > 4)
            .map(|(card, count)| DeckValidationError::TooManyCopies(card.name.clone(), count));
        errors.extend(count_errors);
        if errors.is_empty() { Ok(()) }
        else { Err(errors) }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Card{
    pub name: String,
    pub supertype: Vec<String>,
}

impl Card {
    fn is_basic_land(&self) -> bool {self.supertype.contains(&"basic land".to_string())}
}




/* not sure if this is needed
pub enum BasicLandType {
    Plains, 
    Island, 
    Swamp, 
    Mountain, 
    Forest, 
}
*/

