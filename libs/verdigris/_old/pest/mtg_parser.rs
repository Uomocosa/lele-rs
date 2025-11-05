// use pest::iterators::Pair;
use pest::Parser;
use pest_derive;

#[derive(pest_derive::Parser)]
#[grammar = "./mtg_grammar.pest"] 
struct MtgParser;

#[test]
fn tests() {
    // Let's test two cards
    let llanowar_elves_text = "{T}: Add {G}.";
    let shivan_dragon_text = "Flying\n{R}: Shivan Dragon gets +1/+0 until end of turn.";

    println!("--- Parsing Llanowar Elves ---");
    parse_oracle_text(llanowar_elves_text);
    
    println!("\n--- Parsing Shivan Dragon ---");
    parse_oracle_text(shivan_dragon_text);
}

fn parse_oracle_text(text: &str) {
    // Call the parser. `Rule::oracle_text` is the "entry point" of our grammar.
    match MtgParser::parse(Rule::oracle_text, text) {
        Ok(pairs) => {
            // `pairs` is an iterator over the top-level rules matched.
            // In our case, it should be a single `Rule::oracle_text` pair.
            for pair in pairs {
                // Uncomment this to see the full parse tree
                println!("{:#?}", pair); 

                // We want to dig into the `ability_line`s
                for ability_line in pair.into_inner() {
                    if let Rule::ability_line = ability_line.as_rule() {
                        // Get the *actual* ability inside the line
                        // (e.g., keyword_ability or activated_ability)
                        let inner_ability = ability_line.into_inner().next().unwrap();
                        
                        // Match on the rule type
                        match inner_ability.as_rule() {
                            Rule::keyword_ability => {
                                println!("Found Keyword: {}", inner_ability.as_str());
                            }
                            Rule::activated_ability => {
                                // We can dig even deeper!
                                let mut inner_pairs = inner_ability.into_inner();
                                
                                let cost = inner_pairs.next().unwrap();
                                let effect = inner_pairs.next().unwrap();

                                println!("Found Activated Ability:");
                                println!("  Cost: {}", cost.as_str());
                                println!("  Effect: {}", effect.as_str());
                            }
                            Rule::static_ability => {
                                println!("Found Static Ability: {}", inner_ability.as_str());
                            }
                            // These are helper rules, we don't expect them here
                            _ => println!("Unexpected rule: {:?}", inner_ability.as_rule()),
                        }
                    }
                }
            }
        }
        Err(e) => { println!("Parse Error: {e}"); }
    }
}