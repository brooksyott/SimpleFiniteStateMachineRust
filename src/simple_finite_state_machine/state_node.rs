use crate::simple_finite_state_machine::shared_types::ExecCallback;
use crate::simple_finite_state_machine::state_node::transition::Transition;

/// =================================================================================
/// state_node
/// The node that encapsulates the behaviours of a specific state
/// Each state defined in the state machine will have a state node
/// Provides a fluent API approach
/// =================================================================================
#[path = "shared_types.rs"]
mod shared_types;
#[path = "transition.rs"]
mod transition;

#[derive(Clone)]
pub struct StateNode<TStates, TTriggers> {
    state: TStates, // State value of this node

    on_entry_action: Vec<ExecCallback<TTriggers>>, // Functions to call when entering this state
    on_exit_action: Vec<ExecCallback<TTriggers>>,  // Functions to call when exiting this state

    transitions: Vec<Transition<TStates, TTriggers>>, // Allowed transitions
}

impl<TStates, TTriggers: PartialEq> StateNode<TStates, TTriggers> {
    /// Initialize the state node
    pub fn configure(state: TStates) -> Self {
        StateNode {
            state,
            on_entry_action: Vec::new(),
            on_exit_action: Vec::new(),
            transitions: Vec::new(),
        }
    }

    /// Return the state associated with the node
    pub fn get_state(&self) -> &TStates {
        &self.state
    }

    /// Add a function to be called upon entering this state
    pub fn on_entry(&mut self, cb: ExecCallback<TTriggers>) -> &mut Self {
        self.on_entry_action.push(cb);
        self
    }

    /// Add a function to be called upon exiting this state
    pub fn on_exit(&mut self, cb: ExecCallback<TTriggers>) -> &mut Self {
        self.on_exit_action.push(cb);
        self
    }

    /// Determine the next allowed state, based on the trigger and the guard
    pub fn next_state(&self, trigger: &TTriggers) -> Result<&TStates, String> {
        if self.transitions.len() > 0 {
            for transition in self.transitions.iter() {
                if Self::enums_equal(trigger, &transition.trigger) {
                    let guard_passed = (transition.guard)(trigger);
                    if guard_passed {
                        return Ok(&transition.state);
                    }
                }
            }
        }

        Err("Transition not found".to_string())
    }

    /// Perform a comparison of the generic enum types
    fn enums_equal<T: PartialEq>(a: &T, b: &T) -> bool {
        a.eq(b)
    }

    /// Add a permitted trigger to transition out of this state, with no guard
    pub fn permit(&mut self, trigger: TTriggers, state: TStates) -> &mut Self {
        self.permit_if(trigger, state, Self::empty_guard);
        self
    }

    /// Add a permitted trigger to transition out of this state, with a specified guard
    pub fn permit_if(
        &mut self,
        trigger: TTriggers,
        state: TStates,
        guard: shared_types::GuardCallback<TTriggers>,
    ) -> &mut Self {
        let new_transition = Transition {
            trigger: trigger,
            state: state,
            guard: guard,
        };

        self.transitions.push(new_transition);
        self
    }

    /// A default guard that always returns true
    fn empty_guard(trigger: &TTriggers) -> bool {
        _ = trigger;
        true
    }

    /// Invoke functions we are entering this state
    pub fn entering_state(&self, trigger: &TTriggers) {
        if self.on_entry_action.len() > 0 {
            for action in self.on_entry_action.iter() {
                action(trigger);
            }
        }
    }

    /// Invoke functions we are exiting this state
    pub fn exiting_state(&self, trigger: &TTriggers) {
        if self.on_exit_action.len() > 0 {
            for action in self.on_exit_action.iter() {
                action(trigger);
            }
        }
    }
}
