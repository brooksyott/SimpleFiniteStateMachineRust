use crate::simple_finite_state_machine::finite_state_machine::FiniteStateMachine;
mod simple_finite_state_machine;
use std::fmt::Display;
use enum_display::EnumDisplay;

#[derive(Debug, EnumDisplay, PartialEq, Eq, Hash, Clone)]
pub enum PhoneTriggers {
    None,
    CallDialed,
    TakeOffHook,
    CallConnected,
    LeftMessage,
    PlacedOnHold,
    TakenOffHold,
    Hangup,
    PhoneHurledAgainstWall,
    MuteMicrophone,
    UnmuteMicrophone,
    SetVolume
}

#[derive(Debug, EnumDisplay, PartialEq, Eq, Hash, Clone)]
pub enum PhoneStates {
    OnHook,
    OffHook,
    Ringing,
    Connected,
    OnHold,
    PhoneDestroyed
}

fn main() {

    let mut fsm = FiniteStateMachine::<PhoneStates, PhoneTriggers>::new(&PhoneStates::OnHook);
    fsm.configure(PhoneStates::OnHook).unwrap()
        .on_entry(test_entry_callback_onhook)
        .on_exit(exiting_onhook)
        .permit(PhoneTriggers::TakeOffHook, PhoneStates::OffHook);


    fsm.configure(PhoneStates::OffHook).unwrap()
        .on_entry(test_entry_callback_offhook);


    let state = fsm.get_current_state().unwrap();
    println!("Current state 1: {}", state);

    let trans_result = fsm.fire(&PhoneTriggers::TakeOffHook);
    match trans_result {
        Ok(_) => println!("State transitioned"),
        Err(_) => {
            println!("State transitioned FAILED");
            return;
        },
    };

    let state = fsm.get_current_state().unwrap();
    println!("Current state 2: {}", state);

}

pub fn test_entry_callback_onhook<PhoneTriggers: Display>(trigger: &PhoneTriggers)
{
    println!("Entering: PhoneStates::OnHook - 1, Trigger: {}", trigger);
}

pub fn test_entry_callback_onhook2<TTriggers: Display>(trigger: &TTriggers)
{
    println!("Entering: PhoneStates::OnHook - 2, Trigger: {}", trigger);
}

pub fn test_entry_callback_offhook<TTriggers: Display>(trigger: &TTriggers)
{
    println!("Entering: PhoneStates::OnHook - 1, Trigger: {}", trigger);
}

pub fn exiting_onhook<TTriggers: Display>(trigger: &TTriggers)
{
    println!("Exiting: PhoneStates::OnHook, Trigger: {}", trigger);
}