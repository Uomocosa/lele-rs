#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    White,
    Blue,
    Black,
    Red,
    Green,
    Generic(u8),
    Colorless,
    Phyrexian,
    Snow,
    Tap, // duplicate
    Untap, // duplicate
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mana<'a>(pub &'a [Symbol]);

#[derive(Clone)]
pub enum Cost<'a> {
    Tap,
    Untap,
    Mana(&'a [Mana<'a>]), 
}

pub const W: Mana = Mana(&[Symbol::White]);
pub const U: Mana = Mana(&[Symbol::Blue]);
pub const B: Mana = Mana(&[Symbol::Black]);
pub const R: Mana = Mana(&[Symbol::Red]);
pub const G: Mana = Mana(&[Symbol::Green]);
pub const C: Mana = Mana(&[Symbol::Colorless]);
pub const S: Mana = Mana(&[Symbol::Snow]);

pub const WP: Mana = Mana(&[Symbol::White, Symbol::Phyrexian]);
pub const UP: Mana = Mana(&[Symbol::Blue, Symbol::Phyrexian]);
pub const BP: Mana = Mana(&[Symbol::Black, Symbol::Phyrexian]);
pub const RP: Mana = Mana(&[Symbol::Red, Symbol::Phyrexian]);
pub const GP: Mana = Mana(&[Symbol::Green, Symbol::Phyrexian]);
pub const CP: Mana = Mana(&[Symbol::Colorless, Symbol::Phyrexian]);

pub const W2: Mana = Mana(&[Symbol::White, Symbol::Generic(2)]);
pub const U2: Mana = Mana(&[Symbol::Blue, Symbol::Generic(2)]);
pub const B2: Mana = Mana(&[Symbol::Black, Symbol::Generic(2)]);
pub const R2: Mana = Mana(&[Symbol::Red, Symbol::Generic(2)]);
pub const G2: Mana = Mana(&[Symbol::Green, Symbol::Generic(2)]);
pub const C2: Mana = Mana(&[Symbol::Colorless, Symbol::Generic(2)]);

// ...You can continue if you want

// This are not correct ;; 
// Mana should have a source and properties.
// Snow is a property of mana
// pub const SW: Mana = Mana(&[Symbol::Snow, Symbol::White]);
// pub const SU: Mana = Mana(&[Symbol::Snow, Symbol::Blue]);
// pub const SB: Mana = Mana(&[Symbol::Snow, Symbol::Black]);
// pub const SR: Mana = Mana(&[Symbol::Snow, Symbol::Red]);
// pub const SG: Mana = Mana(&[Symbol::Snow, Symbol::Green]);
// pub const SC: Mana = Mana(&[Symbol::Snow, Symbol::Colorless]);


#[test]
pub fn test() {
    let _red_mana = W;
}