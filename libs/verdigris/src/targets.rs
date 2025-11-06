#[derive(From)]
pub enum Target {
    Player(PlayerTarget),
    Object(Object),
    Zone(Zone),
    Effect(Effect), // For Counterspells or effects that copy a spell
    Spell,
    Itself,        // For costs/effects that refer to the card the rules text is on
}

#[derive(From)]
pub enum PlayerTarget {
    You,
    Opponent,
    AnyPlayer,
    TargetPlayer, // When the player is chosen upon casting/activation
}

#[derive(From)]
pub enum ObjectTarget {
    AnyPermanent,
    AnyObject,
    OfType(CardType), // e.g., Creature, Artifact, Enchantment
    WithFilter(Filter), // e.g., "Nonland permanent," "Red creature," "Tapped creature"
    ByName(String),
    CreatureWithPower(Comparison), // e.g., "creature with power 4 or greater"
    LandOfBasicType(LandType),     // e.g., "Island"
}

#[derive(From)]
pub enum CardType {
    Creature,
    Land,
    Artifact,
    Enchantment,
    Planeswalker,
    Instant,
    Sorcery,
    Battle,
    // ... all other types/supertypes
}

#[derive(From)]
pub struct Comparison {
    pub operator: ComparisonOperator, // e.g., GreaterThan, EqualTo
    pub value: i32,
    pub property: Property,           // e.g., Power, Toughness, ManaValue
}
pub enum ComparisonOperator { TODO }
pub enum Property { TODO }





