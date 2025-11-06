use crate::Card::Card;

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub enum Zone{
//     Library,
//     Hand,
//     Battlefield,
//     Graveyard,
//     Stack,
//     Exile,
//     Ante,
//     Command,
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Library(pub Vec<Card>);
