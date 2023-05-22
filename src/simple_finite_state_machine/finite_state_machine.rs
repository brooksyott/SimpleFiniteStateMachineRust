use crate::simple_finite_state_machine::state::State;
use dict::{ Dict, DictIface };
use std::collections::HashMap;

struct MyStruct {
    // Define your struct fields here
    field1: u32,
    field2: String,
}

pub struct FiniteStateMachine<TStates>
{
    // state: State<TStates>,
    // current_state: State<TStates>,

    states: HashMap<TStates, State<TStates>>
    // states: HashMap<TStates, MyStruct>
}

impl<TStates> FiniteStateMachine<TStates>
    where
        TStates: std::cmp::Eq + std::hash::Hash, TStates: Clone
{
    pub fn new(initial_state: TStates) -> Self
    {
        FiniteStateMachine {
            // state: State::configure(initial_state.clone()),
            // current_state: State::configure(initial_state.clone()),
            states: HashMap::new()
         }
    }

    pub fn configure(&mut self, new_state: TStates) -> Option<&mut State<TStates>> {
        let mut tstate = State::configure(new_state.clone());
        // let value = MyStruct {field1: 10, field2: "dave".to_string() };
        self.states.insert(new_state.clone(), tstate);
        return self.states.get_mut(&new_state.clone());
    }

    pub fn get_state(&mut self, state: &TStates) -> Option<&mut State<TStates>> {
        return self.states.get_mut(state);
    }
}