use crate::simple_finite_state_machine::finite_state_machine::FiniteStateMachine;

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

pub struct PhoneFsm<PhoneTriggers> {
    fsm: FiniteStateMachine::<PhoneStates, PhoneTriggers>,
}

impl PhoneFsm<PhoneTriggers> {
    pub fn new() -> Self {
        PhoneFsm {
            fsm: FiniteStateMachine::<PhoneStates, PhoneTriggers>::new(&PhoneStates::OnHook)
        }
    }

    pub fn build(&mut self) {

        self.fsm.configure(PhoneStates::OnHook).unwrap()
            .on_entry(Self::on_hook)
            .permit(PhoneTriggers::TakeOffHook, PhoneStates::OffHook);

        self.fsm.configure(PhoneStates::OffHook).unwrap()
            .permit(PhoneTriggers::CallDialed, PhoneStates::Ringing)
            .permit(PhoneTriggers::Hangup, PhoneStates::OnHook);

        self.fsm.configure(PhoneStates::Ringing).unwrap()
            .on_entry(Self::phone_ringing)
            .permit(PhoneTriggers::CallConnected, PhoneStates::Connected)
            .permit(PhoneTriggers::Hangup, PhoneStates::OnHook);

        self.fsm.configure(PhoneStates::Connected).unwrap()
            .on_entry(Self::phone_connected)
            .on_exit(Self::phone_disconnected)
            .permit(PhoneTriggers::Hangup, PhoneStates::OnHook)
            .permit(PhoneTriggers::MuteMicrophone, PhoneStates::Connected)
            .permit(PhoneTriggers::UnmuteMicrophone, PhoneStates::Connected)
            .permit(PhoneTriggers::SetVolume, PhoneStates::Connected);
    }

    pub fn on_hook<TTriggers: Display>(trigger: &TTriggers) {
        println!("Callback: On Hook, Trigger: {}", trigger);
    }

    pub fn phone_ringing<TTriggers: Display>(trigger: &TTriggers) {
        println!("Callback: Phone Ringing, Trigger: {}", trigger);
    }

    pub fn phone_connected<PhoneTriggers: Display>(trigger: &PhoneTriggers) {
        println!("Callback: Phone connected, Trigger: {}", trigger);
    }

    pub fn phone_disconnected<PhoneTriggers: Display>(trigger: &PhoneTriggers) {
        println!("Callback: Phone disconnected, Trigger: {}", trigger);
    }

    pub fn make_call(&mut self) {
        println!("+++ make_call: Dialing");
        let phone_result = self.fsm.fire(&PhoneTriggers::CallDialed);
        match phone_result {
            Ok(_) => {
                let state = self.fsm.get_current_state().unwrap();
                println!("--- make_call: New State: {}", state);
            },
            Err(e) => {
                println!("--- Error: {}", e.to_string());
                return;
            },
        };
    }

    pub fn mute_call(&mut self) {
        println!("+++ mute_call: Mute");
        let phone_result = self.fsm.fire(&PhoneTriggers::MuteMicrophone);
        match phone_result {
            Ok(_) => {
                let state = self.fsm.get_current_state().unwrap();
                println!("--- mute_call: New State: {}", state);
            },
            Err(e) => {
                println!("--- Error: {}", e.to_string());
                return;
            },
        };
    }

    pub fn connect_call(&mut self) {
        println!("+++ connect_call: Answered");
        let phone_result = self.fsm.fire(&PhoneTriggers::CallConnected);
        match phone_result {
            Ok(_) => {
                let state = self.fsm.get_current_state().unwrap();
                println!("--- connect_call: New State: {}", state);
            },
            Err(e) => {
                println!("--- Error: {}", e.to_string());
                return;
            },
        };
    }

    pub fn take_off_hook(&mut self) {
        println!("+++ take_off_hook: Taking phone off hook");
        let phone_result = self.fsm.fire(&PhoneTriggers::TakeOffHook);
        match phone_result {
            Ok(_) => {
                let state = self.fsm.get_current_state().unwrap();
                println!("--- take_off_hook: New State: {}", state);
            },
            Err(e) => {
                println!("--- Error: {}", e.to_string());
                return;
            },
        };
    }

    pub fn hang_up(&mut self) {
        println!("+++ hang_up: Hanging up");
        let phone_result = self.fsm.fire(&PhoneTriggers::Hangup);
        match phone_result {
            Ok(_) => {
                let state = self.fsm.get_current_state().unwrap();
                println!("--- hang_up: New State: {}", state);
            },            Err(e) => {
                println!("--- Error: {}", e.to_string());
                return;
            },
        };
    }
}



pub fn do_phone_call_1() {
    let mut phone = PhoneFsm::<PhoneTriggers>::new();
    phone.build();
    phone.connect_call();
    phone.take_off_hook();
    phone.make_call();
    phone.connect_call();
    phone.mute_call();
    phone.hang_up();
}