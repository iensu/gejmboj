//! Sharp SM83 CPU implementation

use crate::{
    errors::CpuError, instructions, instructions::Instruction, memory::Memory, registers::Registers,
};

#[allow(non_snake_case)]
#[derive(Debug, PartialEq)]
pub struct CpuFlags {
    /// Interrupt Master Enable
    ///
    /// Enables interrupts based on the current state of the IE register (memory address 0xFFFF)
    pub IME: bool,

    /// If true at the start of a machine cycle IME should be enabled
    pub IME_scheduled: bool,
}

impl CpuFlags {
    pub fn new() -> Self {
        Self {
            IME: false,
            IME_scheduled: false,
        }
    }
}

pub struct CPU {
    flags: CpuFlags,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            flags: CpuFlags::new(),
        }
    }

    pub fn tick(
        &mut self,
        registers: &mut Registers,
        memory: &mut Memory,
    ) -> Result<(u16, Instruction), CpuError> {
        let opcode = memory.get(registers.PC.into());
        let instruction_location = registers.PC.clone();

        let instruction = instructions::decode(opcode, registers.PC.into(), memory)?;

        registers.PC += instruction.length();

        instruction.execute(registers, memory, &mut self.flags)?;

        Ok((instruction_location, instruction))
    }
}
