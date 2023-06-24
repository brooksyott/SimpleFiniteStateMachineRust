#![allow(dead_code)]

use std::any::Any;

/// =================================================================================
/// Shared Types
/// A set of types shared across this module
/// =================================================================================

pub type GuardCallback<TTriggers> = fn(trigger: &TTriggers) -> bool;
pub type ExecCallback<TTriggers> = fn(trigger: &TTriggers);
pub type ClosureCallBack = fn(f: fn());
