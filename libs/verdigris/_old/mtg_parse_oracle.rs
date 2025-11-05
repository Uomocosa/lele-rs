use thiserror::Error; // 1. Import the Error derive macro

#[derive(Debug, Error, PartialEq, Eq)] 
pub enum ParseError {
    #[error("Deck must contain at least 60 cards, but found {0}")]
    LessThan60Cards(usize),

    #[error("Deck contains more than 4 copies of card: '{0}' (found {1})")]
    TooManyCopies(String, usize), // (Example of a 2nd error type)
}

#[derive(Debug, PartialEq, Eq)] 
pub struct Effect{}

fn mtg_parse_oracle(oracle: &str) -> Result<Vec<Effect>, ParseError> {
    Ok(vec![Effect{}])
}


#[cfg(test)]
mod tests {
    use super::*;

    fn lionware_elf() {
        let oracle = "{T}: Add {G}";
        let effect = mtg_parse_oracle(oracle);
        assert_eq!(effect, Ok(vec![Effect{}]));
    }
}