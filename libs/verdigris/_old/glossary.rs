// Define an empty struct (a "unit struct") to act as a namespace
#[derive(Debug, PartialEq, Eq)] // Allow printing and comparing
pub struct One;

// Now, we implement "associated items" for 'One'
impl One {
    // Inside 'One', we define another unit struct 'Two'
    #[derive(Debug, PartialEq, Eq)]
    pub struct Two;
}

// We can even implement items for 'One::Two'
impl One::Two {
    // And inside 'One::Two', we define 'Three'
    #[derive(Debug, PartialEq, Eq)]
    pub struct Three;
    
    // We could also add a 'Four'
    #[derive(Debug, PartialEq, Eq)]
    pub struct Four;
}


// pub enum Mana {
//     Red,
//     White,
//     // Snow,
// }

// pub enum Phyrexian {}
// pub struct Hybrid;

// impl Mana {
//     pub const Phyrexian: Phyrexian = Phyrexian;    
//     pub const Hybrid: Hybrid = Hybrid;
// }

// impl Phyrexian {
//     pub const Red: Mana = Mana::Phyrexian;
// }


fn test() {
    let cost = vec![
        Mana::Red,
        Mana::Snow,
        // Mana::Snow(White),
        Mana::Phyrexian,
        Mana::Hybrid(vec![Mana::Red, Mana::White]),
        Mana::Phyrexian::Hybrid(vec![Mana::Red, Mana::White]),
    ];
}