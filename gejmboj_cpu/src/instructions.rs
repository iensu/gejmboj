//! Sharp SM83 instruction set

use crate::combine_instructions;
use crate::{errors::CpuError, memory::Memory, registers::Registers};

pub mod control_flow;
pub mod load;
pub mod misc;

use control_flow::ControlFlow;
use load::Load8Bit;
use misc::Misc;

/// Return either the number of consumed machine cycles, or a `CpuError`.
pub type InstructionResult = Result<u16, CpuError>;

combine_instructions! {
    Instruction(ControlFlow, Load8Bit, Misc)
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

/// Decode an operation code into an `Instruction`.
pub fn decode(opcode: u8, pc: u16, memory: &Memory) -> Result<Instruction, CpuError> {
    match into_bits(opcode) {
        // ABSOLUTE MATCHES
        //
        // misc
        (0, 0, 0, 0, 0, 0, 0, 0) => Ok(Instruction::Misc(Misc::Noop())),

        // control flow
        (1, 1, 0, 0, 0, 0, 1, 1) => Ok(Instruction::ControlFlow(ControlFlow::Jp(
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, 0, 0, 1, 0, 0, 1) => Ok(Instruction::ControlFlow(ControlFlow::Ret())),
        (1, 1, 0, 1, 1, 0, 0, 1) => Ok(Instruction::ControlFlow(ControlFlow::RetI())),
        (1, 1, 0, 0, 1, 1, 0, 1) => Ok(Instruction::ControlFlow(ControlFlow::Call(
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, 1, 0, 1, 0, 0, 1) => Ok(Instruction::ControlFlow(ControlFlow::JpToHL())),
        (0, 0, 0, 1, 1, 0, 0, 0) => Ok(Instruction::ControlFlow(ControlFlow::JpToOffset(
            get_8bit_operand(pc, memory),
        ))),

        // 8 bit load instructions
        (0, 0, 0, 0, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdBCToA())),
        (0, 0, 0, 1, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdDEToA())),
        (0, 0, 0, 0, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdAToBC())),
        (0, 0, 0, 1, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdAToDE())),
        (1, 1, 1, 1, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdToA(get_16bit_operand(
            pc, memory,
        )))),
        (1, 1, 1, 1, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdhCToA())),
        (1, 1, 1, 0, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdhAToC())),
        (1, 1, 1, 1, 0, 0, 0, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdhToA(get_8bit_operand(
            pc, memory,
        )))),
        (1, 1, 1, 0, 0, 0, 0, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdhFromA(
            get_8bit_operand(pc, memory),
        ))),
        (1, 1, 1, 0, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdFromA(
            get_16bit_operand(pc, memory),
        ))),
        (0, 0, 1, 1, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdAFromHLDec())),
        (0, 0, 1, 1, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdAToHLDec())),
        (0, 0, 1, 0, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdAFromHLInc())),
        (0, 0, 1, 0, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdAToHLInc())),

        // VARIABLE MATCHES
        //
        // control flow
        (1, 1, 0, c, d, 0, 1, 0) => Ok(Instruction::ControlFlow(ControlFlow::JpIf(
            get_16bit_operand(pc, memory),
            Condition::parse(c, d).unwrap(),
        ))),
        (0, 0, 1, c, d, 0, 0, 0) => Ok(Instruction::ControlFlow(ControlFlow::JpToOffsetIf(
            get_8bit_operand(pc, memory),
            Condition::parse(c, d).unwrap(),
        ))),
        (1, 1, 0, c, d, 1, 0, 0) => Ok(Instruction::ControlFlow(ControlFlow::CallIf(
            get_16bit_operand(pc, memory),
            Condition::parse(c, d).unwrap(),
        ))),
        (1, 1, 0, c, d, 0, 0, 0) => Ok(Instruction::ControlFlow(ControlFlow::RetIf(
            Condition::parse(c, d).unwrap(),
        ))),
        (1, 1, _, _, _, 1, 1, 1) => Ok(Instruction::ControlFlow(ControlFlow::Rst(opcode))),

        // 8 bit load instructions
        (0, 1, a, b, c, 1, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LdFromHL((a, b, c).into()))),
        (0, 1, 1, 1, 0, a, b, c) => Ok(Instruction::Load8Bit(Load8Bit::LdToHL((a, b, c).into()))),
        (0, 1, a, b, c, x, y, z) => Ok(Instruction::Load8Bit(Load8Bit::Ld(
            (a, b, c).into(),
            (x, y, z).into(),
        ))),

        // Catch all
        _ => Err(CpuError::UnknownInstruction(opcode)),
    }
}
