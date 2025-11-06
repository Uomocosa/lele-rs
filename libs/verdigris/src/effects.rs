use winnow::ModalResult;

pub enum Effect {
    GainLife(u32, Target),                  // e.g., "Gain 5 life."
    LoseLife(u32, Target),                  // e.g., "You lose 2 life."
    AddCounter(CounterType, u32, Target),   // e.g., "Put a +1/+1 counter on target creature."
    ModifyHandSize(u32, Target),            // e.g., "Maximum hand size is reduced by 2." (Static Effect)

    Destroy(Target),                        // e.g., "Destroy target permanent."
    Exile(Target, ZoneTarget),              // e.g., "Exile target creature until ~ leaves the battlefield."
    Bounce(Target),                         // e.g., "Return target permanent to its owner's hand."
    Tap(Target),                            // e.g., "Tap target permanent."
    Untap(Target),                          // e.g., "Untap target permanent."
    ChangeControl(Target, PlayerTarget),    // e.g., "Gain control of target creature until end of turn."
    ModifyPT(Target, i32, i32, Duration),   // e.g., "Creature gets +2/+2 until end of turn."

    Draw(u32, Target),                      // e.g., "Draw 3 cards."
    Discard(i32, Target),                   // e.g., "Discard a card at random."
    SearchLibrary(PlayerTarget, Target),    // e.g., "Search your library for a Forest"
    ShuffleLibrary(Target),
    ChangeZone(Target, ZoneTarget, ZoneTarget), // e.g., "Put target card from your graveyard into your hand."

    Mill(i32, Target),                      // e.g., "Mill 3 cards."
    Scry(i32, Target),                      // e.g., "Scry 2."
    CreateToken(TokenDetails),              // e.g., "Create a 1/1 white Soldier creature token."
    GrantAbility(Target, Ability, Duration), // e.g., "Creature gains Flying until end of turn."
    Counter(Target),                        // e.g., "Counter target spell."
}


pub fn parse_effect(input: &mut &'static str) -> ModalResult<Effect> {
    unimplemented!()
}