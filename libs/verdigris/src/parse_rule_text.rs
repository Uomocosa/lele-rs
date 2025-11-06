use winnow::{ModalResult, Parser, combinator::alt};

use crate::{activated_abilities::{self, ActivatedAbility, parse_activated_ability}, costs::Cost, effects::Effect, symbols::Symbol};

pub enum Action {
    Draw(u8),
    Discard(u8),
    Mill(u8),
}

pub struct TriggeredAbility {
    trigger: Action,
    effect: Effect
}

#[derive(From)]
pub enum Ability {
    Activated(ActivatedAbility),
    Triggered(TriggeredAbility),
    Static(Effect),
}

#[derive(From)]
pub enum RuleText {
    Keyword(&'static [RuleText]), // I'm not sure about this
    Modal(&'static [RuleText]), // I dont like this
    AlternativeCasting(Cost),
    AdditionalCost(Cost),
    Ability(Ability),
    OneShot(Effect),
    Replacement(Effect),
    Prevention(Effect),
}

#[derive(From)]
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


pub fn parse_rule_text(input: &mut &'static str) -> ModalResult<RuleText> {
    alt((
        parse_activated_ability.map(std::convert::Into::into),
    ))
    .parse_next(input)
}