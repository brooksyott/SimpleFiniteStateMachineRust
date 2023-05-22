/// =================================================================================
/// Transition
/// Foundation for transitions
/// =================================================================================

#[path = "shared_types.rs"]
mod shared_types;

#[derive(Clone)]
pub struct Transition<TStates, TTriggers> {
    pub trigger: TTriggers, // the trigger to move to the new state
    pub state: TStates,     // The state to which we will transition
    pub guard: shared_types::GuardCallback<TTriggers>, // A function/guard, that if true permits the transition to the new state
}
