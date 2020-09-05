use super::Instruction;
use crate::{errors::CpuError, memory::Memory, registers::Registers};
use std::fmt::Display;

/// No operation
pub struct Noop {}

impl Instruction for Noop {
    fn execute(&self, _registers: &mut Registers, _memory: &mut Memory) -> Result<(), CpuError> {
        Ok(())
    }
    fn duration(&self) -> u16 {
        1
    }
    fn length(&self) -> u16 {
        1
    }
}

impl Display for Noop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NOOP")
    }
}
