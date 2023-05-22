#[path = "shared_types.rs"] mod shared_types;

pub struct InternalTransition<TTriggers>
{
    pub trigger: TTriggers,
    pub exec: shared_types::ExecCallback,
    pub guard: shared_types::GuardCallback
}