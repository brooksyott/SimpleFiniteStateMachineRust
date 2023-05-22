#![allow(dead_code)]
/// =================================================================================
/// Finite State Machine (FSM)
/// Implements the basics of a simple finite state machine
/// Supports:
/// - multiple callbacks when entering a state
/// - multiple callbacks when exiting a state
/// - setting up custom states and triggers
/// - configuring guards
/// =================================================================================
use crate::simple_finite_state_machine::state_node::StateNode;
use std::collections::HashMap;

/// Basic structure of the state machine
/// current_state: current state of the machine
/// state_nodes: all possible states and their configuration
pub struct FiniteStateMachine<TStates, TTriggers> {
    state_nodes: HashMap<TStates, StateNode<TStates, TTriggers>>,
    current_state: TStates,
    // states: HashMap<TStates, MyStruct>
}

impl<TStates, TTriggers: PartialEq> FiniteStateMachine<TStates, TTriggers>
where
    TStates: std::cmp::Eq + std::hash::Hash,  // Allow for comparing of generic types
    TStates: Clone,
{
    /// Initialize the state machine, with the initial state
    pub fn new(initial_state: &TStates) -> Self {
        FiniteStateMachine {
            // state: State::configure(initial_state.clone()),
            // current_state: State::configure(initial_state.clone()),
            current_state: initial_state.clone(),
            state_nodes: HashMap::new(),
        }
    }

    /// Configure a new state
    /// This uses a fluent design pattern, and as a result returns a mutable reference
    /// to the state node being configured
    /// This allows transitions, guards etc.. to be setup in the state node
    pub fn configure(&mut self, new_state: TStates) -> Option<&mut StateNode<TStates, TTriggers>> {
        let tstate = StateNode::configure(new_state.clone());
        self.state_nodes.insert(new_state.clone(), tstate);
        return self.state_nodes.get_mut(&new_state);
    }

    /// Cause a trigger to be fired
    pub fn fire(&mut self, trigger: &TTriggers) -> Result<(), String> {
        let trans_result = self.transition_states(&trigger);
        let result = match trans_result {
            Ok(r) => r,
            Err(err) => return Err(err),
        };

        if result != true {
            return Err("Could not transition state, return code false".to_string());
        }

        Ok(())
    }

    /// Transition between states based on the trigger
    fn transition_states(&mut self, trigger: &TTriggers) -> Result<bool, String> {
        let state_node_option = self.state_nodes.get(&self.current_state);
        let state_node = match state_node_option {
            None => {println!("Current state not found"); return Err("Current state not found".to_string());}
            Some(s) => s
        };
        let next_state_result = state_node.next_state(trigger);
        let next_state = match next_state_result {
            Err(e) => { return Err(e.to_string()) }
            Ok(ns) => ns
        };

        // If the next state is the same as the current state,
        // do NOT call the entry and exit
        if Self::enums_equal(next_state, &self.current_state) {
            return Ok(true);
        }

        // call the exiting state actions
        state_node.exiting_state(trigger);

        // set the new state, and call the entering state actions
        self.current_state = next_state.clone();
        let next_state_node_option = self.state_nodes.get(next_state);
        let next_state_node = match next_state_node_option {
            None => {println!("Nothing found"); return Err("Nothing found".to_string());}
            Some(s) => s
        };

        next_state_node.entering_state(&trigger);

        Ok(true)
    }

    /// Perform a comparison of the generic enum types
    fn enums_equal<T: PartialEq>(a: &T, b: &T) -> bool {
        a.eq(b)
    }

    /// Allows the user to gain access to any state node
    pub fn get_state(&mut self, state: &TStates) -> Option<&mut StateNode<TStates, TTriggers>> {
        return self.state_nodes.get_mut(&state);
    }

    /// Exposes the current state value
    pub fn get_current_state(&mut self) -> Option<&TStates> {
        let state_node_option = self.state_nodes.get_mut(&self.current_state);
        match state_node_option {
            Some(t) => return Some(&t.get_state()),
            _ => return None,
        }
    }

    /// Exposes the state node associated with the current state
    pub fn get_current_state_node(&mut self) -> Option<&mut StateNode<TStates, TTriggers>> {
        return self.state_nodes.get_mut(&self.current_state);
    }
}
