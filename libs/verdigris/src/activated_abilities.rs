use winnow::{ModalResult, Parser, combinator::separated_pair, token::literal};
use crate::{costs::{Cost, parse_cost}, effects::{Effect, parse_effect}};

pub struct ActivatedAbility{
    pub cost: Vec<Cost>,
    pub effect: Effect,
}

pub fn parse_activated_ability(input: &mut &'static str) -> ModalResult<ActivatedAbility> {
    separated_pair(
        parse_cost, 
        literal(":"), 
        parse_effect, 
    )
        .map(|(cost, effect)| {
            ActivatedAbility { cost, effect }
        })
        .parse_next(input) // <-- The parent parser calls `.parse_next()` ONCE
}
