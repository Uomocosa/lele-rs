#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol<'a> {
    Hybrid(&'a [Symbol<'a>]),
    White,
    Blue,
    Black,
    Red,
    Green,
    Generic(u8),
    Colorless,
    Phyrexian,
    Snow,
    Tap,
    Untap,
    Infinity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cost<'a> {
    Symbol(&'a [Symbol<'a>]),
    Action
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Draw(u8),
    Discard(u8),
    Mill(u8),
}

pub const W: Symbol = Symbol::White;
pub const U: Symbol = Symbol::Blue;
pub const B: Symbol = Symbol::Black;
pub const R: Symbol = Symbol::Red;
pub const G: Symbol = Symbol::Green;
pub const C: Symbol = Symbol::Colorless;
pub const P: Symbol = Symbol::Phyrexian;
pub const S: Symbol = Symbol::Snow;
pub const T: Symbol = Symbol::Tap;
pub const Q: Symbol = Symbol::Untap;
// pub const INF: Symbol = Symbol::Infinity; // added to soon

pub const WP: Symbol = Symbol::Hybrid(&[W, P]);
pub const UP: Symbol = Symbol::Hybrid(&[U, P]);
pub const BP: Symbol = Symbol::Hybrid(&[B, P]);
pub const RP: Symbol = Symbol::Hybrid(&[R, P]);
pub const GP: Symbol = Symbol::Hybrid(&[G, P]);
pub const CP: Symbol = Symbol::Hybrid(&[C, P]);

pub const W2: Symbol = Symbol::Hybrid(&[W, Symbol::Generic(2)]);
pub const U2: Symbol = Symbol::Hybrid(&[U, Symbol::Generic(2)]);
pub const B2: Symbol = Symbol::Hybrid(&[B, Symbol::Generic(2)]);
pub const R2: Symbol = Symbol::Hybrid(&[R, Symbol::Generic(2)]);
pub const G2: Symbol = Symbol::Hybrid(&[G, Symbol::Generic(2)]);
pub const C2: Symbol = Symbol::Hybrid(&[C, Symbol::Generic(2)]);

// ...You can continue if you want
