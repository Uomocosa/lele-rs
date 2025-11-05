// use pest::iterators::Pair;
// use pest::Parser;
use pest_derive;
use pest_ast::FromPest;
use std::str::FromStr;


#[derive(pest_derive::Parser)]
#[grammar = "./mtg_grammar.pest"] 
struct MtgParser;


#[test]
fn tests() {
    // Let's test two cards
    let llanowar_elves_text = "{T}: Add {G}.";
    let shivan_dragon_text = "Flying\n{R}: Shivan Dragon gets +1/+0 until end of turn.";

    println!("--- Parsing Llanowar Elves ---");
    // parse_oracle_text(llanowar_elves_text);
    
    println!("\n--- Parsing Shivan Dragon ---");
    // parse_oracle_text(shivan_dragon_text);
}

// // Links to the .pest file
// #[derive(pest_derive::Parser)]
// #[grammar = "oracle.pest"]
// pub struct OracleParser;


// // --- Top-Level Ability ---
// // This enum maps to the `ability` rule
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::ability))]
// pub enum ParsedAbility {
//     // This variant maps to the `add_ability` rule
//     #[pest_ast(rule(Rule::add_ability))]
//     Add(ParsedManaAmount), // It automatically finds `mana_amount` inside

//     // This variant maps to the `draw_ability` rule
//     #[pest_ast(rule(Rule::draw_ability))]
//     Draw(ParsedDrawCards), // It automatically finds `draw_ability`
// }

// // --- Mana Amount ---
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::mana_amount))] // Maps to the `mana_amount` rule
// pub struct ParsedManaAmount {
//     // The `+` in the rule is understood.
//     // This vec is filled with `mana_symbol` results.
//     pub symbols: Vec<ParsedManaSymbol>,
// }

// // --- Draw Cards ---
// #[derive(Debug, FromPest)]
// #[pest_ast(rule(Rule::draw_ability))] // Maps to the `draw_ability` rule
// pub struct ParsedDrawCards {
//     // This maps the `number` rule and uses FromStr
//     // to parse the string "2" into the u8 2.
//     #[pest_ast(outer(with(FromStr::from_str)))]
//     pub count: u8,
// }

// --- Mana Symbol Enum ---
// This is the most "inline" part, just as you wanted.
#[derive(Debug, Clone, FromPest)]
// #[pest_ast(rule(Rule::mana_symbol))] // Maps to the `mana_symbol` rule
pub enum ParsedManaSymbol {
    // This is your #[search_for("{R}")]
    #[pest_ast(rule(Rule::r_mana))]
    Red,
    #[pest_ast(rule(Rule::g_mana))]
    Green,
    #[pest_ast(rule(Rule::b_mana))]
    Black,
    #[pest_ast(rule(Rule::w_mana))]
    White,
    #[pest_ast(rule(Rule::u_mana))]
    Blue,
    #[pest_ast(rule(Rule::c_mana))]
    Colorless,
    #[pest_ast(rule(Rule::x_mana))]
    X,
    // This variant implicitly maps to the `generic_mana` rule
    Generic(ParsedGenericMana),
}

#[derive(Debug, FromPest, Clone)]
#[pest_ast(rule(Rule::generic_mana))] // Maps to `generic_mana` rule
pub struct ParsedGenericMana {
    // Maps the `number` rule inside `generic_mana`
    #[pest_ast(outer(with(FromStr::from_str)))]
    pub value: u8,
}