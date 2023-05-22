mod simple_finite_state_machine;
use crate::simple_finite_state_machine::finite_state_machine::FiniteStateMachine;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum PhoneStates {
    OnHook,
    OffHook,
    Ringing,
    Connected,
    OnHold,
    PhoneDestroyed
}

fn main() {
    let mut fsm = FiniteStateMachine::<PhoneStates>::new(PhoneStates::OnHook);
    let mut k = fsm.configure(PhoneStates::OnHook).unwrap()
                                        .inc_trigger()
                                        .inc_trigger();
    println!("state = {:?}", k);
    println!("state = {:?}", k);
    let s = fsm.get_state(&PhoneStates::OnHook);
    println!("state = {:?}", s);
    println!("Hello, world!");
}
