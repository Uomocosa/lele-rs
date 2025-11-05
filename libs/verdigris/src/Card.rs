use core::str;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Power(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Toughness(pub u32);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequiresCapability(pub String); // "Flight"

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvidesCapability(pub String); // "Flight"

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TriggeredAbility{
    pub trigger: Trigger,
    pub effect: Effect,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Condition{}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Trigger{
    OnResolve,
    OnCast,
    When(Condition),
    Whenever(Condition),
    As(Condition),
    If(Condition),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect{}

