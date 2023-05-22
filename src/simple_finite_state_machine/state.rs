use std::fmt;

#[path = "shared_types.rs"] mod shared_types;

#[derive(Debug)]
pub struct State<TStates>
{
    state: TStates,
    num_triggers: u32,

    on_entry_action: Vec<shared_types::ExecCallback>,
    on_exit_action: Vec<shared_types::ExecCallback>
}

impl<TStates> State<TStates> {
    // pub fn new() -> Self
    // {
    //      State {
    //          num_triggers: 0,
    //          on_entry_action: Vec::new(),
    //          on_exit_action: Vec::new()
    //      }
    // }

    pub fn configure(state: TStates) -> Self
    {
        State {
            state: state,
            num_triggers: 0,
            on_entry_action: Vec::new(),
            on_exit_action: Vec::new()
        }
    }

    pub fn inc_trigger(&mut self) -> &mut Self
    {
        self.num_triggers += 1;
        self
    }
}
