pub struct Game {
    players: Vec<Player>,
}

pub struct Player {
    id: PlayerId,
    role: PlayerType,
    zone: Zone,
}

pub enum PlayerRole {
    You,
    Ally,
    Opponent,
}

pub struct Zone {
    battlefield: Battlefield,
    graveyard: Graveyard,
    library: Library,
}

pub struct Battlefield(Vec<Card>);
pub struct Graveyard(Vec<Card>);
pub struct Library(Vec<Card>);

pub struct Card {
    name: String,
    mana_cost: Vec<Symbol>,
    cmc: u8,
    oracle: Vec<RuleText>, // this should probably be Vec<Vec<>>
    r#type: String, // type is a reserved keyword in rust.
    supertype: Option<String>,
    power: Option<u8>,
    toughness: Option<u8>,
    flavor_text: String,
}


pub enum RuleText { TODO };

pub enum Symbol { TODO };