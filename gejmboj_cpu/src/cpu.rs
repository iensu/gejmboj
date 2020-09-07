//! Sharp SM83 CPU implementation

use crate::{
    errors::CpuError, instructions, instructions::Instruction, memory::Memory, registers::Registers,
};

#[allow(non_snake_case)]
pub struct CpuFlags {
    pub IME: u8,
}

impl CpuFlags {
    pub fn new() -> Self {
        Self { IME: 0 }
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
    ) -> Result<(u16, Box<dyn Instruction>), CpuError> {
        let opcode = memory.get(registers.PC.into());
        let instruction_location = registers.PC.clone();

        let instruction = instructions::decode(opcode, registers.PC.into(), memory)?;

        registers.PC += instruction.length();

        instruction.execute(registers, memory, &mut self.flags)?;

        Ok((instruction_location, instruction))
    }
}
