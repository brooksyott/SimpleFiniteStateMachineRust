#![allow(dead_code)]
/// =================================================================================
/// Shared Types
/// A set of types shared across this module
/// =================================================================================

pub type GuardCallback<TTriggers> = fn(trigger: &TTriggers) -> bool;
pub type ExecCallback<TTriggers> = fn(trigger: &TTriggers);
