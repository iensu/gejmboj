//! Sharp SM83 CPU implementation

use crate::{
    errors::CpuError, instructions, instructions::Instruction, memory::Memory, registers::Registers,
};

#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Eq)]
pub struct CpuFlags {
    /// Interrupt Master Enable
    ///
    /// Enables interrupts based on the current state of the IE register (memory address 0xFFFF)
    pub IME: bool,

    /// If true at the start of a machine cycle IME should be enabled
    pub IME_scheduled: bool,
}

impl Default for CpuFlags {
    fn default() -> Self {
        Self::new()
    }
}

impl CpuFlags {
    #[must_use]
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

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    #[must_use]
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
        let instruction_location = registers.PC;

        let instruction = instructions::decode(opcode, registers.PC, memory)?;

        registers.PC += instruction.length();

        if self.flags.IME_scheduled {
            self.flags.IME = true;
            self.flags.IME_scheduled = false;
        }

        instruction.execute(registers, memory, &mut self.flags)?;

        Ok((instruction_location, instruction))
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use instructions::Instruction;
    use instructions::control_flow;
    use instructions::misc;

    #[test]
    fn cpu_tick_executes_instructiona() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        let mut cpu = CPU::new();

        let noop = 0b0000_0000;
        memory.set_u16(0x0000, noop);

        assert_eq!(0, registers.PC);

        let (_, instruction) = cpu.tick(&mut registers, &mut memory).unwrap();

        assert_eq!(Instruction::Misc(misc::Misc::NOP()), instruction);
        assert_eq!(instruction.length(), registers.PC);
    }

    #[test]
    fn cpu_tick_handles_interrupt_scheduling() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        let mut cpu = CPU::new();

        let ei_op = 0b1111_1011;
        let noop = 0b0000_0000;

        memory.set_u16(0x0000, ei_op);
        memory.set_u16(0x0002, noop);

        let (_, instruction) = cpu
            .tick(&mut registers, &mut memory)
            .expect("Failed to execute EI instruction");

        assert_eq!(Instruction::Misc(misc::Misc::EI()), instruction);
        assert_eq!(instruction.length(), registers.PC);
        assert_eq!(
            CpuFlags {
                IME: false,
                IME_scheduled: true
            },
            cpu.flags
        );

        cpu.tick(&mut registers, &mut memory)
            .expect("Failed to execute Noop instruction");

        assert_eq!(
            CpuFlags {
                IME: true,
                IME_scheduled: false,
            },
            cpu.flags
        );
    }

    #[test]
    fn pc_remains_unchanged_after_jr_fe_instructions() {
        let mut registers = Registers::new();
        let mut memory = Memory::new();
        let mut cpu = CPU::new();

        let jr_fe = 0xFE_18; // as BE bytes

        let prev_pc = registers.PC;

        memory.set_u16(0x0000, jr_fe);

        let (_, instruction) = cpu
            .tick(&mut registers, &mut memory)
            .expect("Failed to execute JR instruction");

        assert_eq!(
            Instruction::ControlFlow(control_flow::ControlFlow::JR(0xFE)),
            instruction
        );
        assert_eq!(prev_pc, registers.PC);
    }
}
