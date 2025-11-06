use winnow::{ModalResult, Parser, ascii::digit1, combinator::alt, error::ContextError, token::literal};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    Choice(&'static [Symbol]),
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
    Generic(u8),
    X,
    Y,
    Z,
    Phyrexian,
    Snow,
    Tap,
    Untap,
    Infinity,
    //...to continue
}



pub const W: Symbol = Symbol::White;
pub const U: Symbol = Symbol::Blue;
pub const B: Symbol = Symbol::Black;
pub const R: Symbol = Symbol::Red;
pub const G: Symbol = Symbol::Green;
pub const C: Symbol = Symbol::Colorless;
pub const X: Symbol = Symbol::X;
pub const Y: Symbol = Symbol::Y;
pub const Z: Symbol = Symbol::Z;
pub const P: Symbol = Symbol::Phyrexian;
pub const S: Symbol = Symbol::Snow;
pub const T: Symbol = Symbol::Tap;
pub const Q: Symbol = Symbol::Untap;
pub const INF: Symbol = Symbol::Infinity;

pub const WP: Symbol = Symbol::Choice(&[W, P]);
pub const UP: Symbol = Symbol::Choice(&[U, P]);
pub const BP: Symbol = Symbol::Choice(&[B, P]);
pub const RP: Symbol = Symbol::Choice(&[R, P]);
pub const GP: Symbol = Symbol::Choice(&[G, P]);
pub const CP: Symbol = Symbol::Choice(&[C, P]);

pub const W2: Symbol = Symbol::Choice(&[Symbol::Generic(2), W]);
pub const U2: Symbol = Symbol::Choice(&[Symbol::Generic(2), U]);
pub const B2: Symbol = Symbol::Choice(&[Symbol::Generic(2), B]);
pub const R2: Symbol = Symbol::Choice(&[Symbol::Generic(2), R]);
pub const G2: Symbol = Symbol::Choice(&[Symbol::Generic(2), G]);
pub const C2: Symbol = Symbol::Choice(&[Symbol::Generic(2), C]);

pub const CW: Symbol = Symbol::Choice(&[C, W]);
pub const CU: Symbol = Symbol::Choice(&[C, U]);
pub const CB: Symbol = Symbol::Choice(&[C, B]);
pub const CR: Symbol = Symbol::Choice(&[C, R]);
pub const CG: Symbol = Symbol::Choice(&[C, G]);

pub const WU: Symbol = Symbol::Choice(&[W, U]);
pub const WB: Symbol = Symbol::Choice(&[W, B]);
pub const WR: Symbol = Symbol::Choice(&[W, R]);
pub const WG: Symbol = Symbol::Choice(&[W, G]);

pub const WUB: Symbol = Symbol::Choice(&[W, U, B]);
pub const WUR: Symbol = Symbol::Choice(&[W, U, R]);
pub const WUG: Symbol = Symbol::Choice(&[W, U, G]);
// ...to continue



/// Parses a single MTG symbol.
///
/// # Errors
/// Returns an error if the input string does NOT start with 
/// - a reconzied symbol character (e.g., "W", "U", "B", "R", "G")
/// - numbers(u8) are accepted as well ("1", "2", ..., "15", "16", ...)
pub fn parse_symbol_ascii(input: &mut &'static str) -> ModalResult<Symbol> {
    alt((
        literal("W").value(W),
        literal("U").value(U),
        literal("B").value(B),
        literal("R").value(R),
        literal("G").value(G),
        literal("C").value(C),
        digit1.map(|num_str: &str| {
            Symbol::Generic(num_str.parse().unwrap_or(0))
        }),
        literal("X").value(X),
        literal("Y").value(Y),
        literal("Z").value(Z),
        literal("P").value(P),
        literal("S").value(S),
        literal("T").value(T),
        literal("Q").value(Q),
        literal("∞").value(Symbol::Infinity),
        //...to_continue
    )).parse_next(input)
}
