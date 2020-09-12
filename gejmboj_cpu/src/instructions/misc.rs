use super::{Instruction, InstructionResult};
use crate::{cpu::CpuFlags, define_instruction, memory::Memory, registers::Registers};
use std::fmt::Display;

define_instruction! {
    /// No operation
    Noop { "NOOP"; 1 }

    (self) => {
        Ok(1)
    }
}
