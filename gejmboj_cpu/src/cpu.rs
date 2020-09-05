use crate::{
    errors::CpuError, instructions, instructions::Instruction, memory::Memory, registers::Registers,
};

pub struct CPU {}

impl CPU {
    pub fn tick(
        &self,
        registers: &mut Registers,
        memory: &mut Memory,
    ) -> Result<(u16, Box<dyn Instruction>), CpuError> {
        let opcode = memory.get(registers.pc.into());
        let instruction = instructions::decode(opcode, registers.pc.into(), memory)?;
        let instruction_location = registers.pc.clone();

        registers.pc += instruction.length();

        instruction.execute(registers, memory)?;

        Ok((instruction_location, instruction))
    }
}
