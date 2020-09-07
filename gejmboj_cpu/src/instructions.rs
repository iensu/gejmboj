//! Sharp SM83 instruction set

use crate::{errors::CpuError, memory::Memory, registers::Registers};

mod control_flow;
mod misc;

pub use control_flow::*;
pub use misc::*;

/// Return either the number of consumed machine cycles, or a `CpuError`.
pub type InstructionResult = Result<u16, CpuError>;

/// Trait for implementing a Sharp SM83 instruction.
pub trait Instruction {
    fn execute(&self, registers: &mut Registers, memory: &mut Memory) -> InstructionResult;

    /// Returns the byte length of the operation
    fn length(&self) -> u16;
}

#[derive(Debug)]
pub enum Condition {
    Carry,
    NoCarry,
    Zero,
    NotZero,
}

impl Condition {
    pub fn parse(c1: u8, c2: u8) -> Result<Self, CpuError> {
        match (c1, c2) {
            (0, 0) => Ok(Condition::Carry),
            (0, 1) => Ok(Condition::NoCarry),
            (1, 0) => Ok(Condition::Zero),
            (1, 1) => Ok(Condition::NotZero),
            _ => Err(CpuError::Error(format!(
                "Unknown instruction condition ({}, {})",
                c1, c2
            ))),
        }
    }

    pub fn is_fulfilled(&self, registers: &Registers) -> bool {
        match self {
            Condition::Carry => registers.is_carry(),
            Condition::NoCarry => !registers.is_carry(),
            Condition::Zero => registers.is_zero(),
            Condition::NotZero => !registers.is_zero(),
        }
    }
}

/// Decode an operation code into an `Instruction`.
pub fn decode(opcode: u8, pc: u16, memory: &Memory) -> Result<Box<dyn Instruction>, CpuError> {
    match into_bits(opcode) {
        (0, 0, 0, 0, 0, 0, 0, 0) => Ok(Box::new(misc::Noop {})),
        (1, 1, 0, 0, 0, 0, 1, 1) => Ok(Box::new(control_flow::Jp {
            operand: get_16bit_operand(pc, memory),
        })),
        (1, 1, 0, 0, 1, 0, 0, 1) => Ok(Box::new(control_flow::Ret {})),
        (1, 1, 0, 0, 1, 1, 0, 1) => Ok(Box::new(control_flow::Call {
            operand: get_16bit_operand(pc, memory),
        })),
        (1, 1, 1, 0, 1, 0, 0, 1) => Ok(Box::new(control_flow::JpToHL {})),
        (0, 0, 0, 1, 1, 0, 0, 0) => Ok(Box::new(control_flow::JpToOffset {
            operand: get_8bit_operand(pc, memory),
        })),
        (1, 1, 0, c, d, 0, 1, 0) => Ok(Box::new(control_flow::JpIf {
            operand: get_16bit_operand(pc, memory),
            condition: Condition::parse(c, d).unwrap(),
        })),
        (0, 0, 1, c, d, 0, 0, 0) => Ok(Box::new(control_flow::JpToOffsetIf {
            operand: get_8bit_operand(pc, memory),
            condition: Condition::parse(c, d).unwrap(),
        })),
        (1, 1, 0, c, d, 1, 0, 0) => Ok(Box::new(control_flow::CallIf {
            operand: get_16bit_operand(pc, memory),
            condition: Condition::parse(c, d).unwrap(),
        })),
        (1, 1, 0, c, d, 0, 0, 0) => Ok(Box::new(control_flow::RetIf {
            condition: Condition::parse(c, d).unwrap(),
        })),
        _ => Err(CpuError::UnknownInstruction(opcode)),
    }
}

fn get_8bit_operand(pc: u16, memory: &Memory) -> u8 {
    memory.get((pc as usize) + 1)
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
