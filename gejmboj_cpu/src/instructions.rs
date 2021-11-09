//! Sharp SM83 instruction set

use crate::combine_instructions;
use crate::{errors::CpuError, memory::Memory, registers::Registers};

pub mod alu_8bit;
pub mod control_flow;
pub mod load_16bit;
pub mod load_8bit;
pub mod misc;

use alu_8bit::ALU8Bit;
use control_flow::ControlFlow;
use load_16bit::Load16Bit;
use load_8bit::Load8Bit;
use misc::Misc;

/// Return either the number of consumed machine cycles, or a `CpuError`.
pub type InstructionResult = Result<u16, CpuError>;

combine_instructions! {
    Instruction(ALU8Bit,ControlFlow, Load8Bit, Load16Bit, Misc)
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
        (0, 0, 1, 1, 1, 1, 1, 1) => Ok(Instruction::Misc(Misc::CCF())),
        (0, 0, 1, 1, 0, 1, 1, 1) => Ok(Instruction::Misc(Misc::SCF())),
        (0, 0, 1, 0, 0, 1, 1, 1) => Ok(Instruction::Misc(Misc::DAA())),
        (0, 0, 1, 0, 1, 1, 1, 1) => Ok(Instruction::Misc(Misc::CPL())),

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

        // ALU 8-bit instructions
        (1, 0, 0, 0, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::AddAHL())),
        (1, 1, 0, 0, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::AddAN(get_8bit_operand(
            pc, memory,
        )))),
        (1, 0, 0, 0, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::AdcAHL())),
        (1, 1, 0, 0, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::AdcAN(get_8bit_operand(
            pc, memory,
        )))),

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

        // ALU 8-bit instructions
        (1, 0, 0, 0, 0, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::AddA((a, b, c).into()))),
        (1, 0, 0, 0, 1, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::AdcA((a, b, c).into()))),

        // Catch all
        _ => Err(CpuError::UnknownInstruction(opcode)),
    }
}

#[cfg(test)]
mod tests {
    use crate::registers::{DoubleRegister as DR, SingleRegister as SR};

    use super::Condition as C;
    use super::ControlFlow as CF;
    use super::Instruction as I;

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

        for (code, instruction) in vec![
            // Misc instructions
            (0b00000000, I::Misc(Misc::Noop())),
            (0b00111111, I::Misc(Misc::CCF())),
            (0b00110111, I::Misc(Misc::SCF())),
            (0b00100111, I::Misc(Misc::DAA())),
            (0b00101111, I::Misc(Misc::CPL())),
            (0b11111011, I::Misc(Misc::EI())),
            (0b11110011, I::Misc(Misc::DI())),
            // Control flow instructions
            (0b11000011, I::ControlFlow(CF::Jp(0))),
            (0b11001001, I::ControlFlow(CF::Ret())),
            (0b11011001, I::ControlFlow(CF::RetI())),
            (0b11001101, I::ControlFlow(CF::Call(0))),
            (0b11101001, I::ControlFlow(CF::JpToHL())),
            (0b00011000, I::ControlFlow(CF::JpToOffset(0))),
            (0b11000010, I::ControlFlow(CF::JpIf(0, C::Carry))),
            (0b11011010, I::ControlFlow(CF::JpIf(0, C::NotZero))),
            (0b11000000, I::ControlFlow(CF::RetIf(C::Carry))),
            (0b11011000, I::ControlFlow(CF::RetIf(C::NotZero))),
            (0b00100000, I::ControlFlow(CF::JpToOffsetIf(0, C::Carry))),
            (0b00111000, I::ControlFlow(CF::JpToOffsetIf(0, C::NotZero))),
            (0b11011100, I::ControlFlow(CF::CallIf(0, C::NotZero))),
            (0b11000100, I::ControlFlow(CF::CallIf(0, C::Carry))),
            (0b11000111, I::ControlFlow(CF::Rst(0b11000111))),
            (0b11111111, I::ControlFlow(CF::Rst(0b11111111))),
            // Load 8-bit instructions
            (0b00001010, I::Load8Bit(Load8Bit::LdBCToA())),
            (0b00011010, I::Load8Bit(Load8Bit::LdDEToA())),
            (0b00000010, I::Load8Bit(Load8Bit::LdAToBC())),
            (0b00010010, I::Load8Bit(Load8Bit::LdAToDE())),
            (0b11111010, I::Load8Bit(Load8Bit::LdToA(0))),
            (0b11110010, I::Load8Bit(Load8Bit::LdhCToA())),
            (0b11100010, I::Load8Bit(Load8Bit::LdhAToC())),
            (0b11110000, I::Load8Bit(Load8Bit::LdhToA(0))),
            (0b11100000, I::Load8Bit(Load8Bit::LdhFromA(0))),
            (0b11101010, I::Load8Bit(Load8Bit::LdFromA(0))),
            (0b00111010, I::Load8Bit(Load8Bit::LdAFromHLDec())),
            (0b00110010, I::Load8Bit(Load8Bit::LdAToHLDec())),
            (0b00101010, I::Load8Bit(Load8Bit::LdAFromHLInc())),
            (0b00100010, I::Load8Bit(Load8Bit::LdAToHLInc())),
            (0b01000110, I::Load8Bit(Load8Bit::LdFromHL(SR::B))),
            (0b01111110, I::Load8Bit(Load8Bit::LdFromHL(SR::A))),
            (0b01110000, I::Load8Bit(Load8Bit::LdToHL(SR::B))),
            (0b01110111, I::Load8Bit(Load8Bit::LdToHL(SR::A))),
            (0b01100000, I::Load8Bit(Load8Bit::Ld(SR::H, SR::B))),
            // Load 16-bit instructions
            (0b00000001, I::Load16Bit(Load16Bit::Ld(DR::BC, 0))),
            (0b00010001, I::Load16Bit(Load16Bit::Ld(DR::DE, 0))),
            (0b00100001, I::Load16Bit(Load16Bit::Ld(DR::HL, 0))),
            (0b00110001, I::Load16Bit(Load16Bit::Ld(DR::SP, 0))),
            (0b00001000, I::Load16Bit(Load16Bit::LdFromSP(0))),
            (0b11111001, I::Load16Bit(Load16Bit::LdHLToSP())),
            (0b11000101, I::Load16Bit(Load16Bit::Push(DR::BC))),
            (0b11010101, I::Load16Bit(Load16Bit::Push(DR::DE))),
            (0b11100101, I::Load16Bit(Load16Bit::Push(DR::HL))),
            (0b11110101, I::Load16Bit(Load16Bit::Push(DR::AF))),
            (0b11000001, I::Load16Bit(Load16Bit::Pop(DR::BC))),
            (0b11010001, I::Load16Bit(Load16Bit::Pop(DR::DE))),
            (0b11100001, I::Load16Bit(Load16Bit::Pop(DR::HL))),
            (0b11110001, I::Load16Bit(Load16Bit::Pop(DR::AF))),
            // ALU 8-bit instructions
            (0b10000000, I::ALU8Bit(ALU8Bit::AddA(SR::B))),
            (0b10000001, I::ALU8Bit(ALU8Bit::AddA(SR::C))),
            (0b10000010, I::ALU8Bit(ALU8Bit::AddA(SR::D))),
            (0b10000011, I::ALU8Bit(ALU8Bit::AddA(SR::E))),
            (0b10000100, I::ALU8Bit(ALU8Bit::AddA(SR::H))),
            (0b10000101, I::ALU8Bit(ALU8Bit::AddA(SR::L))),
            (0b10000111, I::ALU8Bit(ALU8Bit::AddA(SR::A))),
            (0b10000110, I::ALU8Bit(ALU8Bit::AddAHL())),
            (0b11000110, I::ALU8Bit(ALU8Bit::AddAN(0))),
            (0b10001000, I::ALU8Bit(ALU8Bit::AdcA(SR::B))),
            (0b10001001, I::ALU8Bit(ALU8Bit::AdcA(SR::C))),
            (0b10001010, I::ALU8Bit(ALU8Bit::AdcA(SR::D))),
            (0b10001011, I::ALU8Bit(ALU8Bit::AdcA(SR::E))),
            (0b10001100, I::ALU8Bit(ALU8Bit::AdcA(SR::H))),
            (0b10001101, I::ALU8Bit(ALU8Bit::AdcA(SR::L))),
            (0b10001111, I::ALU8Bit(ALU8Bit::AdcA(SR::A))),
            (0b10001110, I::ALU8Bit(ALU8Bit::AdcAHL())),
            (0b11001110, I::ALU8Bit(ALU8Bit::AdcAN(0))),
        ] {
            assert_eq!(
                decode(code, pc, &memory).unwrap(),
                instruction,
                "Failed to decode 0b{:08b}",
                code
            );
        }
    }
}
