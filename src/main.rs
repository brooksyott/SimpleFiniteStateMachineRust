// use crate::simple_finite_state_machine::finite_state_machine::FiniteStateMachine;
mod phone_call;
mod simple_finite_state_machine;
use crate::phone_call::build_phone;

fn main() {
    build_phone::do_phone_call_1();
}
