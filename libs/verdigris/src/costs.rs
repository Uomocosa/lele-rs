use winnow::{ModalResult, Parser, ascii::multispace0, error::ContextError};
use crate::{parse_symbol, symbols::Symbol};

#[derive(From)]
pub enum Cost {
    Symbol(Symbol),
    Effect(Effect),
}


pub fn parse_cost(input: &mut &'static str) -> ModalResult<Vec<Cost>> {
    unimplemented!()
}