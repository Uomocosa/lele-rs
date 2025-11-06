use winnow::prelude::*;
use winnow::token::{literal, take_while};
use winnow::combinator::{alt, preceded, terminated, separated, dispatch};
use winnow::ascii::{digit1, multispace0};
use winnow::stream::AsChar;
use crate::glossary::{
    ActivatedAbility, Effect, Mana
};


fn mtg_oracle_parser(input: &mut &'static str) -> Effect {
    
}


fn parse_mana_symbol(input: &mut &'static str) -> ModalResult<Mana> {
    alt((
        literal("{R}").value(Mana::Red),
        literal("{G}").value(Mana::Green),
        literal("{B}").value(Mana::Black),
        literal("{W}").value(Mana::White),
        literal("{U}").value(Mana::Blue),
        literal("{C}").value(Mana::Colorless),
        literal("{X}").value(Mana::X),
        preceded(
            literal("{"), 
            terminated(digit1, literal("}"))
        ).map(|num_str: &str| {
            Mana::Generic(num_str.parse().unwrap_or(0))
        })
    )).parse_next(input)
}

fn parse_mana_amount(input: &mut &'static str) -> ModalResult<Vec<Mana>> {
    // Parse 1 or more `parse_mana_symbol`, separated by 0 spaces
    separated(
        1.., 
        parse_mana_symbol, 
        multispace0
    ).parse_next(input)
}

fn parse_activated_ability(input: &mut &'static str) -> ModalResult<ActivatedAbility> {
    unimplemented!()
}

// fn parse_draw_cards(input: &mut &'static str) -> IResult<DrawCards> {
//     // "Draw "
//     preceded(
//         (tag("Draw"), multispace0), 
//         // "3"
//         digit1
//     )
//     // " card(s)"
//     .terminated((multispace0, tag("card"), tag("s").opt()))
//     .map(|num_str: &str| DrawCards {
//         count: num_str.parse().unwrap_or(0),
//     })
//     .parse_next(input)
// }


// fn parse_add_ability(input: &mut &'static str) -> IResult<AbilityEffect> {
//     preceded(
//         (tag("Add"), multispace0),
//         parse_mana_amount
//     )
//     .map(AbilityEffect::Add) // Directly constructs the clean enum
//     .parse_next(input)
// }

// fn parse_draw_ability(input: &mut &'static str) -> IResult<AbilityEffect> {
//     parse_draw_cards
//         .map(AbilityEffect::Draw) // Directly constructs the clean enum
//         .parse_next(input)
// }

/* 
// This is your main parser function.
// It tries `parse_add_ability`, and if that fails,
// it tries `parse_draw_ability`.
pub fn parse_ability(text: &'static str) -> Result<AbilityEffect, String> {
    // We need a mutable reference to the input string
    let mut input = text; 
    
    // `alt` tries a list of parsers
    let parser = alt((
        // parse_add_ability,
        // parse_draw_ability,
    ));

    // Run the parser
    parser.parse_next(&mut input)
        .map_err(|e| e.to_string())
        .and_then(|result| {
            // Check that we consumed the *entire* input
            if input.is_empty() {
                Ok(result)
            } else {
                Err(format!("Parser finished, but input remains: {}", input))
            }
        })
}
*/