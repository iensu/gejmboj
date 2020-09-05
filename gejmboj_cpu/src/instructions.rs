use crate::{errors::CpuError, memory::Memory, registers::Registers};

mod control_flow;
mod misc;

pub use control_flow::*;
pub use misc::*;

pub trait Instruction {
    fn execute(&self, registers: &mut Registers, memory: &mut Memory) -> Result<(), CpuError>;

    /// Returns the number of machine cycles required to execute the instruction
    fn duration(&self) -> u16;

    /// Returns the byte length of the operation
    fn length(&self) -> u16;
}

pub fn decode(opcode: u8, pc: u16, memory: &Memory) -> Result<Box<dyn Instruction>, CpuError> {
    match into_bits(opcode) {
        (0, 0, 0, 0, 0, 0, 0, 0) => Ok(Box::new(misc::Noop {})),
        (1, 1, 0, 0, 0, 0, 1, 1) => Ok(Box::new(control_flow::Jp {
            operand: get_16bit_operand(pc, memory),
        })),

        _ => Err(CpuError::UnknownInstruction(opcode)),
    }
}

fn get_16bit_operand(pc: u16, memory: &Memory) -> u16 {
    memory.get_u16((pc as usize) + 1)
}

fn into_bits(x: u8) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    (
        x & 0b1000_0000,
        x & 0b0100_0000,
        x & 0b0010_0000,
        x & 0b0001_0000,
        x & 0b0000_1000,
        x & 0b0000_0100,
        x & 0b0000_0010,
        x & 0b0000_0001,
    )
}
