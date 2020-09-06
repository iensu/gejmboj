use super::Instruction;
use std::fmt::Display;

/// Unconditional jump to location specified by 16-bit operand.
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = Jp { operand: 0xBADA };
///
/// assert_eq!(0, registers.pc);
/// instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0xBADA, registers.pc);
/// ```
pub struct Jp {
    pub operand: u16,
}

impl Instruction for Jp {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> Result<(), crate::errors::CpuError> {
        registers.pc = self.operand;
        Ok(())
    }
    fn duration(&self) -> u16 {
        4
    }
    fn length(&self) -> u16 {
        3
    }
}

impl Display for Jp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP({:04x})", self.operand)
    }
}
