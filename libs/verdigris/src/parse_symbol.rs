use winnow::{ModalResult, Parser, ascii::{multispace0}, combinator::{preceded, repeat, separated, terminated}, token::literal};

#[allow(clippy::wildcard_imports)]
use crate::symbols::*;

#[test]
fn example() {
    assert_eq!(parse_symbol(&mut "{W}"), Ok(vec![W]));
    assert_eq!(parse_symbol(&mut "{5}{W}{U}{B}{R}{G}"), Ok(vec![Symbol::Generic(5), W, U, B, R, G]));
    assert_eq!(parse_symbol(&mut "{W/R/P}"), Ok(vec![Symbol::Choice(&[W, R, P])]));
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn parse_single_symbol(input: &mut &'static str) -> ModalResult<Symbol> {
    preceded(
        literal("{"),
        terminated(
            // 1. Parse 1 or more sub-symbols separated by "/"
            separated(1.., parse_symbol_ascii, literal("/")),
            literal("}")
        )
    )
    .map(|mut symbols: Vec<Symbol>| {
        // 2. Analyze the result
        if symbols.len() == 1 {
            // It was {W}, {G}, {3}, etc.
            symbols.pop().unwrap() // Return the single symbol
        } else {
            // It was {W/R}, {W/R/P}, or... {2/R}
            Symbol::Choice(symbols.leak())
        }
    })
    .parse_next(input)
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
#[allow(dead_code)]
pub fn parse_symbol(input: &mut &'static str) -> ModalResult<Vec<Symbol>> {
    // Parse 1 or more `parse_mana_symbol`, separated by 0 spaces
    repeat(
        1.., 
        terminated(parse_single_symbol, multispace0) 
    ).parse_next(input)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn parse_symbol_ascii_00() { assert_eq!(parse_symbol_ascii(&mut "G"), Ok(G)); }
    
    #[test]
    fn parse_symbol_ascii_01() { assert_eq!(parse_symbol_ascii(&mut "2"), Ok(Symbol::Generic(2))); }
    
    #[test]
    fn parse_single_symbol_00() { assert_eq!(parse_single_symbol(&mut "{G}"), Ok(G)); }
    
    #[test]
    fn parse_single_symbol_01() { assert_eq!(parse_single_symbol(&mut "{X}"), Ok(X)); }

    #[test]
    fn parse_symbol_00() { assert_eq!(parse_symbol(&mut "{G}"), Ok(vec![G])); }

    #[test]
    fn parse_symbol_01() { assert_eq!(parse_symbol(&mut "{G}{G}"), Ok(vec![G,G])); }

    #[test]
    fn parse_symbol_03() { assert_eq!(parse_symbol(&mut "{2/R}"), Ok(vec![R2])); }

    #[test]
    fn parse_symbol_04() { assert_eq!(parse_symbol(&mut "{W/R/P}"), Ok(vec![Symbol::Choice(&[W, R, P])])); }

    #[test]
    fn parse_symbol_05() { assert_eq!(parse_symbol(&mut "{W/R/G}"), Ok(vec![Symbol::Choice(&[W, R, G])])); }
    
    #[test]
    fn parse_symbol_06() { 
        assert_eq!(
            parse_symbol(&mut "{W/R/G}{2/R}{C/B}"), 
            Ok(vec![Symbol::Choice(&[W, R, G]), Symbol::Choice(&[Symbol::Generic(2), R]), Symbol::Choice(&[C, B])])
        ); 
    }

}
