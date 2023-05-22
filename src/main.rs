use crate::simple_finite_state_machine::finite_state_machine::FiniteStateMachine;
mod simple_finite_state_machine;
mod phone_call;
use crate::phone_call::build_phone;
use std::fmt::Display;
use enum_display::EnumDisplay;

fn main() {
    build_phone::do_phone_call_1();
}
