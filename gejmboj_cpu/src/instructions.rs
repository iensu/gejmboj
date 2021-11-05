//! Sharp SM83 instruction set

use crate::combine_instructions;
use crate::{errors::CpuError, memory::Memory, registers::Registers};

pub mod control_flow;
pub mod load_16bit;
pub mod load_8bit;
pub mod misc;

use control_flow::ControlFlow;
use load_16bit::Load16Bit;
use load_8bit::Load8Bit;
use misc::Misc;

/// Return either the number of consumed machine cycles, or a `CpuError`.
pub type InstructionResult = Result<u16, CpuError>;

combine_instructions! {
    Instruction(ControlFlow, Load8Bit, Load16Bit, Misc)
}

#[derive(Debug, PartialEq)]
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
        (x & 0b1000_0000) >> 7,
        (x & 0b0100_0000) >> 6,
        (x & 0b0010_0000) >> 5,
        (x & 0b0001_0000) >> 4,
        (x & 0b0000_1000) >> 3,
        (x & 0b0000_0100) >> 2,
        (x & 0b0000_0010) >> 1,
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
        (1, 1, 1, 1, 0, 0, 1, 1) => Ok(Instruction::Misc(Misc::DI())),
        (1, 1, 1, 1, 1, 0, 1, 1) => Ok(Instruction::Misc(Misc::EI())),

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
        (0, 0, 0, 0, 1, 0, 0, 0) => Ok(Instruction::Load16Bit(Load16Bit::LdFromSP(
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, 1, 1, 1, 0, 0, 1) => Ok(Instruction::Load16Bit(Load16Bit::LdHLToSP())),

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

        // 16 bit load instructions
        (0, 0, a, b, 0, 0, 0, 1) => Ok(Instruction::Load16Bit(Load16Bit::Ld(
            (0, a, b).into(),
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, a, b, 0, 1, 0, 1) => Ok(Instruction::Load16Bit(Load16Bit::Push((1, a, b).into()))),
        (1, 1, a, b, 0, 0, 0, 1) => Ok(Instruction::Load16Bit(Load16Bit::Pop((1, a, b).into()))),

        // Catch all
        _ => Err(CpuError::UnknownInstruction(opcode)),
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::DoubleRegister;

    use super::*;

    #[test]
    fn into_bits_works() {
        assert_eq!(into_bits(0b1000_0000), (1, 0, 0, 0, 0, 0, 0, 0));
        assert_eq!(into_bits(0b0100_0000), (0, 1, 0, 0, 0, 0, 0, 0));
        assert_eq!(into_bits(0b0010_0000), (0, 0, 1, 0, 0, 0, 0, 0));
        assert_eq!(into_bits(0b0001_0000), (0, 0, 0, 1, 0, 0, 0, 0));
        assert_eq!(into_bits(0b0000_1000), (0, 0, 0, 0, 1, 0, 0, 0));
        assert_eq!(into_bits(0b0000_0100), (0, 0, 0, 0, 0, 1, 0, 0));
        assert_eq!(into_bits(0b0000_0010), (0, 0, 0, 0, 0, 0, 1, 0));
        assert_eq!(into_bits(0b0000_0001), (0, 0, 0, 0, 0, 0, 0, 1));
        assert_eq!(into_bits(0b0000_0000), (0, 0, 0, 0, 0, 0, 0, 0));
        assert_eq!(into_bits(0b1111_1111), (1, 1, 1, 1, 1, 1, 1, 1));
        assert_eq!(into_bits(0b1000_1000), (1, 0, 0, 0, 1, 0, 0, 0));
    }

    #[test]
    fn decode_works() {
        let memory = Memory::new();
        let pc = 0;

        assert_eq!(
            decode(0b00000000, pc, &memory).unwrap(),
            Instruction::Misc(Misc::Noop())
        );
        assert_eq!(
            decode(0b11000011, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::Jp(get_16bit_operand(pc, &memory)))
        );
        assert_eq!(
            decode(0b11001001, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::Ret())
        );
        assert_eq!(
            decode(0b11011001, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::RetI())
        );
        assert_eq!(
            decode(0b11001101, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::Call(get_16bit_operand(pc, &memory)))
        );
        assert_eq!(
            decode(0b11101001, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::JpToHL())
        );
        assert_eq!(
            decode(0b00011000, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::JpToOffset(get_8bit_operand(pc, &memory)))
        );
        assert_eq!(
            decode(0b00001010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdBCToA())
        );
        assert_eq!(
            decode(0b00011010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdDEToA())
        );
        assert_eq!(
            decode(0b00000010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdAToBC())
        );
        assert_eq!(
            decode(0b00010010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdAToDE())
        );
        assert_eq!(
            decode(0b11111010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdToA(get_16bit_operand(pc, &memory)))
        );
        assert_eq!(
            decode(0b11110010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdhCToA())
        );
        assert_eq!(
            decode(0b11100010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdhAToC())
        );
        assert_eq!(
            decode(0b11110000, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdhToA(get_8bit_operand(pc, &memory)))
        );
        assert_eq!(
            decode(0b11100000, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdhFromA(get_8bit_operand(pc, &memory)))
        );
        assert_eq!(
            decode(0b11101010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdFromA(get_16bit_operand(pc, &memory)))
        );
        assert_eq!(
            decode(0b00111010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdAFromHLDec())
        );
        assert_eq!(
            decode(0b00110010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdAToHLDec())
        );
        assert_eq!(
            decode(0b00101010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdAFromHLInc())
        );
        assert_eq!(
            decode(0b00100010, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdAToHLInc())
        );
        assert_eq!(
            decode(0b11000010, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::JpIf(
                get_16bit_operand(pc, &memory),
                Condition::parse(0, 0).unwrap()
            ))
        );
        assert_eq!(
            decode(0b11011010, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::JpIf(
                get_16bit_operand(pc, &memory),
                Condition::parse(1, 1).unwrap()
            ))
        );
        assert_eq!(
            decode(0b00100000, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::JpToOffsetIf(
                get_8bit_operand(pc, &memory),
                Condition::parse(0, 0).unwrap()
            ))
        );
        assert_eq!(
            decode(0b00111000, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::JpToOffsetIf(
                get_8bit_operand(pc, &memory),
                Condition::parse(1, 1).unwrap()
            ))
        );
        assert_eq!(
            decode(0b11000100, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::CallIf(
                get_16bit_operand(pc, &memory),
                Condition::parse(0, 0).unwrap()
            ))
        );
        assert_eq!(
            decode(0b11011100, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::CallIf(
                get_16bit_operand(pc, &memory),
                Condition::parse(1, 1).unwrap()
            ))
        );
        assert_eq!(
            decode(0b11000000, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::RetIf(Condition::parse(0, 0).unwrap()))
        );
        assert_eq!(
            decode(0b11011000, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::RetIf(Condition::parse(1, 1).unwrap()))
        );
        assert_eq!(
            decode(0b11000111, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::Rst(0b11000111))
        );
        assert_eq!(
            decode(0b11111111, pc, &memory).unwrap(),
            Instruction::ControlFlow(ControlFlow::Rst(0b11111111))
        );
        assert_eq!(
            decode(0b01000110, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdFromHL((0, 0, 0).into()))
        );
        assert_eq!(
            decode(0b01111110, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdFromHL((1, 1, 1).into()))
        );
        assert_eq!(
            decode(0b01110000, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdToHL((0, 0, 0).into()))
        );
        assert_eq!(
            decode(0b01110111, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::LdToHL((1, 1, 1).into()))
        );
        assert_eq!(
            decode(0b01100000, pc, &memory).unwrap(),
            Instruction::Load8Bit(Load8Bit::Ld((1, 0, 0).into(), (0, 0, 0).into()))
        );
        assert_eq!(
            decode(0b00000001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Ld(
                DoubleRegister::BC,
                get_16bit_operand(pc, &memory)
            ))
        );
        assert_eq!(
            decode(0b00010001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Ld(
                DoubleRegister::DE,
                get_16bit_operand(pc, &memory)
            ))
        );
        assert_eq!(
            decode(0b00100001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Ld(
                DoubleRegister::HL,
                get_16bit_operand(pc, &memory)
            ))
        );
        assert_eq!(
            decode(0b00110001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Ld(
                DoubleRegister::SP,
                get_16bit_operand(pc, &memory)
            ))
        );
        assert_eq!(
            decode(0b00001000, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::LdFromSP(get_16bit_operand(pc, &memory)))
        );
        assert_eq!(
            decode(0b11111001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::LdHLToSP())
        );
        assert_eq!(
            decode(0b11000101, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Push(DoubleRegister::BC))
        );
        assert_eq!(
            decode(0b11010101, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Push(DoubleRegister::DE))
        );
        assert_eq!(
            decode(0b11100101, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Push(DoubleRegister::HL))
        );
        assert_eq!(
            decode(0b11110101, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Push(DoubleRegister::AF))
        );
        assert_eq!(
            decode(0b11000001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Pop(DoubleRegister::BC))
        );
        assert_eq!(
            decode(0b11010001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Pop(DoubleRegister::DE))
        );
        assert_eq!(
            decode(0b11100001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Pop(DoubleRegister::HL))
        );
        assert_eq!(
            decode(0b11110001, pc, &memory).unwrap(),
            Instruction::Load16Bit(Load16Bit::Pop(DoubleRegister::AF))
        );
        assert_eq!(
            decode(0b11111011, pc, &memory).unwrap(),
            Instruction::Misc(Misc::EI()),
        );
        assert_eq!(
            decode(0b11110011, pc, &memory).unwrap(),
            Instruction::Misc(Misc::DI()),
        );
    }
}
