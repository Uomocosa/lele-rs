#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TurnStructure{
    BeginningPhase,
    UntapStep,
    UpkeepStep,
    DrawStep,
    MainPhase,
    CombatPhase,
    BeginningOfCombatStep,
    DeclareAttackersStep,
    DeclareBlockersStep,
    CombatDamageStep,
    EndOfCombatStep,
    EndingPhase,
    EndStep,
    CleanupStep,
}