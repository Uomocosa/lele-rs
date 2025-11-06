#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpecialAction{
    PlayLand,
    TurnCreautureFaceUp,
    TakeActionLater,
    IgnoreSelfStaticAbility,
    CirclingVultures,
    Suspend,
    Companion,
    Fortell,
    RollPlanerDie,
    TurnConspiracyFaceUp,
    Plot,
    PayUnlockCost,
}