#[path = "shared_types.rs"] mod shared_types;


struct Transition<TStates, TTriggers>
{
    pub trigger: TTriggers,
    pub state: TStates,
    pub guard: shared_types::GuardCallback
}