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
            return Err("Some error".to_string());
        }

        Ok(())
    }

    /// Transition between states based on the trigger
    fn transition_states(&mut self, trigger: &TTriggers) -> Result<bool, String> {
        let state_node = self.state_nodes.get(&self.current_state).unwrap();
        let next_state = state_node.next_state(trigger).unwrap();
        let next_state_node = self.state_nodes.get(next_state).unwrap();

        state_node.exiting_state(trigger);
        self.current_state = next_state.clone();
        next_state_node.entering_state(&trigger);
        Ok(true)
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
