//! Sharp SM83 instruction set

use crate::combine_instructions;
use crate::{errors::CpuError, memory::Memory, registers::Registers};

pub mod alu_16bit;
pub mod alu_8bit;
pub mod bit;
pub mod control_flow;
pub mod load_16bit;
pub mod load_8bit;
pub mod misc;
pub mod rotate_shift;
mod utils;

use alu_8bit::ALU8Bit;
use alu_16bit::ALU16Bit;
use bit::Bit;
use control_flow::ControlFlow;
use load_8bit::Load8Bit;
use load_16bit::Load16Bit;
use misc::Misc;
use rotate_shift::RotateShift;
use utils::into_bits;

/// Return either the number of consumed machine cycles, or a `CpuError`.
pub type InstructionResult = Result<u16, CpuError>;

combine_instructions! {
    Instruction(ALU16Bit, ALU8Bit, Bit, ControlFlow, Load8Bit, Load16Bit, Misc, RotateShift)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Condition {
    Carry,
    NoCarry,
    Zero,
    NotZero,
}

impl Condition {
    pub fn parse(c1: u8, c2: u8) -> Result<Self, CpuError> {
        match (c1, c2) {
            (0, 0) => Ok(Self::NotZero),
            (0, 1) => Ok(Self::Zero),
            (1, 0) => Ok(Self::NoCarry),
            (1, 1) => Ok(Self::Carry),
            _ => Err(CpuError::Error(format!(
                "Unknown instruction condition ({c1}, {c2})"
            ))),
        }
    }

    #[must_use]
    pub fn is_fulfilled(&self, registers: &Registers) -> bool {
        match self {
            Self::Carry => registers.is_carry(),
            Self::NoCarry => !registers.is_carry(),
            Self::Zero => registers.is_zero(),
            Self::NotZero => !registers.is_zero(),
        }
    }
}

fn get_8bit_operand(pc: u16, memory: &Memory) -> u8 {
    memory.get(pc + 1)
}

fn get_16bit_operand(pc: u16, memory: &Memory) -> u16 {
    memory.get_u16(pc + 1)
}

/// Decode an operation code into an `Instruction`.
#[allow(clippy::too_many_lines)]
pub fn decode(opcode: u8, pc: u16, memory: &Memory) -> Result<Instruction, CpuError> {
    match into_bits(opcode) {
        // ABSOLUTE MATCHES
        //
        // misc
        (0, 0, 0, 0, 0, 0, 0, 0) => Ok(Instruction::Misc(Misc::NOP())),
        (1, 1, 1, 1, 0, 0, 1, 1) => Ok(Instruction::Misc(Misc::DI())),
        (1, 1, 1, 1, 1, 0, 1, 1) => Ok(Instruction::Misc(Misc::EI())),
        (0, 0, 1, 1, 1, 1, 1, 1) => Ok(Instruction::Misc(Misc::CCF())),
        (0, 0, 1, 1, 0, 1, 1, 1) => Ok(Instruction::Misc(Misc::SCF())),
        (0, 0, 1, 0, 0, 1, 1, 1) => Ok(Instruction::Misc(Misc::DAA())),
        (0, 0, 1, 0, 1, 1, 1, 1) => Ok(Instruction::Misc(Misc::CPL())),

        // control flow
        (1, 1, 0, 0, 0, 0, 1, 1) => Ok(Instruction::ControlFlow(ControlFlow::JP(
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, 0, 0, 1, 0, 0, 1) => Ok(Instruction::ControlFlow(ControlFlow::RET())),
        (1, 1, 0, 1, 1, 0, 0, 1) => Ok(Instruction::ControlFlow(ControlFlow::RETI())),
        (1, 1, 0, 0, 1, 1, 0, 1) => Ok(Instruction::ControlFlow(ControlFlow::CALL(
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, 1, 0, 1, 0, 0, 1) => Ok(Instruction::ControlFlow(ControlFlow::JP_HL())),
        (0, 0, 0, 1, 1, 0, 0, 0) => Ok(Instruction::ControlFlow(ControlFlow::JR(
            get_8bit_operand(pc, memory),
        ))),

        // 8 bit load instructions
        (0, 0, 0, 0, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_BC_TO_A())),
        (0, 0, 0, 1, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_DE_TO_A())),
        (0, 0, 0, 0, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_A_TO_BC())),
        (0, 0, 0, 1, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_A_TO_DE())),
        (1, 1, 1, 1, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_TO_A(
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, 1, 1, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LDH_C_TO_A())),
        (1, 1, 1, 0, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LDH_C_FROM_A())),
        (1, 1, 1, 1, 0, 0, 0, 0) => Ok(Instruction::Load8Bit(Load8Bit::LDH_TO_A(
            get_8bit_operand(pc, memory),
        ))),
        (1, 1, 1, 0, 0, 0, 0, 0) => Ok(Instruction::Load8Bit(Load8Bit::LDH_FROM_A(
            get_8bit_operand(pc, memory),
        ))),
        (1, 1, 1, 0, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_FROM_A(
            get_16bit_operand(pc, memory),
        ))),
        (0, 0, 1, 1, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_A_FROM_HL_DEC())),
        (0, 0, 1, 1, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_A_TO_HL_DEC())),
        (0, 0, 1, 0, 1, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_A_FROM_HL_INC())),
        (0, 0, 1, 0, 0, 0, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_A_TO_HL_INC())),
        (0, 0, 0, 0, 1, 0, 0, 0) => Ok(Instruction::Load16Bit(Load16Bit::LD_FROM_SP(
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, 1, 1, 1, 0, 0, 1) => Ok(Instruction::Load16Bit(Load16Bit::LD_HL_TO_SP())),

        // ALU 8-bit instructions
        (1, 0, 0, 0, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::ADD_HL())),
        (1, 1, 0, 0, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::ADD_N(get_8bit_operand(
            pc, memory,
        )))),
        (1, 0, 0, 0, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::ADC_HL())),
        (1, 1, 0, 0, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::ADC_N(get_8bit_operand(
            pc, memory,
        )))),
        (1, 0, 0, 1, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::SUB_HL())),
        (1, 1, 0, 1, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::SUB_N(get_8bit_operand(
            pc, memory,
        )))),
        (1, 0, 0, 1, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::SBC_HL())),
        (1, 1, 0, 1, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::SBC_N(get_8bit_operand(
            pc, memory,
        )))),
        (1, 0, 1, 0, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::AND_HL())),
        (1, 1, 1, 0, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::AND_N(get_8bit_operand(
            pc, memory,
        )))),
        (1, 0, 1, 1, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::OR_HL())),
        (1, 1, 1, 1, 0, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::OR_N(get_8bit_operand(
            pc, memory,
        )))),
        (1, 0, 1, 0, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::XOR_HL())),
        (1, 1, 1, 0, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::XOR_N(get_8bit_operand(
            pc, memory,
        )))),
        (1, 0, 1, 1, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::CP_HL())),
        (1, 1, 1, 1, 1, 1, 1, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::CP_N(get_8bit_operand(
            pc, memory,
        )))),
        (0, 0, 1, 1, 0, 1, 0, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::INC_HL())),
        (0, 0, 1, 1, 0, 1, 0, 1) => Ok(Instruction::ALU8Bit(ALU8Bit::DEC_HL())),

        // ALU 16-bit instructions
        (1, 1, 1, 0, 1, 0, 0, 0) => Ok(Instruction::ALU16Bit(ALU16Bit::ADD_SP(get_8bit_operand(
            pc, memory,
        )))),

        // Rotate Shift instructions
        (0, 0, 0, 0, 0, 1, 1, 1) => Ok(Instruction::RotateShift(RotateShift::RLCA())),
        (0, 0, 0, 0, 1, 1, 1, 1) => Ok(Instruction::RotateShift(RotateShift::RRCA())),
        (0, 0, 0, 1, 0, 1, 1, 1) => Ok(Instruction::RotateShift(RotateShift::RLA())),
        (0, 0, 0, 1, 1, 1, 1, 1) => Ok(Instruction::RotateShift(RotateShift::RRA())),
        (1, 1, 0, 0, 1, 0, 1, 1) => {
            let operand = get_8bit_operand(pc, memory);

            rotate_shift::decode(operand)
                .map(Instruction::RotateShift)
                .or_else(|_| bit::decode(operand).map(Instruction::Bit))
        }

        // VARIABLE MATCHES
        //
        // control flow
        (1, 1, 0, c, d, 0, 1, 0) => Ok(Instruction::ControlFlow(ControlFlow::JPC(
            get_16bit_operand(pc, memory),
            Condition::parse(c, d).unwrap(),
        ))),
        (0, 0, 1, c, d, 0, 0, 0) => Ok(Instruction::ControlFlow(ControlFlow::JRC(
            get_8bit_operand(pc, memory),
            Condition::parse(c, d).unwrap(),
        ))),
        (1, 1, 0, c, d, 1, 0, 0) => Ok(Instruction::ControlFlow(ControlFlow::CALLC(
            get_16bit_operand(pc, memory),
            Condition::parse(c, d).unwrap(),
        ))),
        (1, 1, 0, c, d, 0, 0, 0) => Ok(Instruction::ControlFlow(ControlFlow::RETC(
            Condition::parse(c, d).unwrap(),
        ))),
        (1, 1, _, _, _, 1, 1, 1) => Ok(Instruction::ControlFlow(ControlFlow::RST(opcode))),

        // 8 bit load instructions
        (0, 1, a, b, c, 1, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_FROM_HL(
            (a, b, c).into(),
        ))),
        (0, 1, 1, 1, 0, a, b, c) => Ok(Instruction::Load8Bit(Load8Bit::LD_TO_HL((a, b, c).into()))),
        (0, 1, x1, y1, z1, x2, y2, z2) => Ok(Instruction::Load8Bit(Load8Bit::LD(
            (x1, y1, z1).into(),
            (x2, y2, z2).into(),
        ))),
        (0, 0, a, b, c, 1, 1, 0) => Ok(Instruction::Load8Bit(Load8Bit::LD_IMM(
            (a, b, c).into(),
            get_8bit_operand(pc, memory),
        ))),

        // 16 bit load instructions
        (0, 0, a, b, 0, 0, 0, 1) => Ok(Instruction::Load16Bit(Load16Bit::LD(
            (0, a, b).into(),
            get_16bit_operand(pc, memory),
        ))),
        (1, 1, a, b, 0, 1, 0, 1) => Ok(Instruction::Load16Bit(Load16Bit::PUSH((1, a, b).into()))),
        (1, 1, a, b, 0, 0, 0, 1) => Ok(Instruction::Load16Bit(Load16Bit::POP((1, a, b).into()))),

        // ALU 8-bit instructions
        (1, 0, 0, 0, 0, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::ADD((a, b, c).into()))),
        (1, 0, 0, 0, 1, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::ADC((a, b, c).into()))),
        (1, 0, 0, 1, 0, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::SUB((a, b, c).into()))),
        (1, 0, 0, 1, 1, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::SBC((a, b, c).into()))),
        (1, 0, 1, 0, 0, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::AND((a, b, c).into()))),
        (1, 0, 1, 1, 0, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::OR((a, b, c).into()))),
        (1, 0, 1, 0, 1, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::XOR((a, b, c).into()))),
        (1, 0, 1, 1, 1, a, b, c) => Ok(Instruction::ALU8Bit(ALU8Bit::CP((a, b, c).into()))),
        (0, 0, a, b, c, 1, 0, 0) => Ok(Instruction::ALU8Bit(ALU8Bit::INC((a, b, c).into()))),
        (0, 0, a, b, c, 1, 0, 1) => Ok(Instruction::ALU8Bit(ALU8Bit::DEC((a, b, c).into()))),

        // ALU 16-bit instructions
        (0, 0, b, c, 1, 0, 0, 1) => Ok(Instruction::ALU16Bit(ALU16Bit::ADD_HL((0, b, c).into()))),
        (0, 0, b, c, 0, 0, 1, 1) => Ok(Instruction::ALU16Bit(ALU16Bit::INC((0, b, c).into()))),
        (0, 0, b, c, 1, 0, 1, 1) => Ok(Instruction::ALU16Bit(ALU16Bit::DEC((0, b, c).into()))),

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
    use super::RotateShift as RS;

    use super::*;

    #[test]
    fn decode_with_operand_rotate_shift_instructions_works() {
        let code = 0b1100_1011;
        let pc = 0;
        let mut memory = Memory::new();

        for (operand, instruction) in [
            (0b0000_0111, I::RotateShift(RS::RLC(0b0000_0111))),
            (0b0000_1111, I::RotateShift(RS::RRC(0b0000_1111))),
            (0b0001_0111, I::RotateShift(RS::RL(0b0001_0111))),
            (0b0001_1111, I::RotateShift(RS::RR(0b0001_1111))),
            (0b0010_0111, I::RotateShift(RS::SLA(0b0010_0111))),
            (0b0010_1111, I::RotateShift(RS::SRA(0b0010_1111))),
            (0b0011_0111, I::RotateShift(RS::SWAP(0b0011_0111))),
            (0b0011_1111, I::RotateShift(RS::SRL(0b0011_1111))),
        ] {
            memory.set(pc + 1, operand);

            assert_eq!(
                instruction,
                decode(code, pc, &memory).unwrap(),
                "Failed to decode with operand 0b{operand:08b}"
            );
        }
    }

    #[test]
    fn decode_with_operand_bit_instructions_works() {
        let code = 0b1100_1011;
        let pc = 0;
        let mut memory = Memory::new();

        for (operand, instruction) in [
            (0b0100_0111, I::Bit(Bit::BIT(0b0100_0111))),
            (0b1100_1111, I::Bit(Bit::SET(0b1100_1111))),
            (0b1001_0111, I::Bit(Bit::RES(0b1001_0111))),
        ] {
            memory.set(pc + 1, operand);

            assert_eq!(
                instruction,
                decode(code, pc, &memory).unwrap(),
                "Failed to decode with operand 0b{operand:08b}"
            );
        }
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn decode_works() {
        let memory = Memory::new();
        let pc = 0;

        for (code, instruction) in vec![
            // Misc instructions
            (0b0000_0000, I::Misc(Misc::NOP())),
            (0b0011_1111, I::Misc(Misc::CCF())),
            (0b0011_0111, I::Misc(Misc::SCF())),
            (0b0010_0111, I::Misc(Misc::DAA())),
            (0b0010_1111, I::Misc(Misc::CPL())),
            (0b1111_1011, I::Misc(Misc::EI())),
            (0b1111_0011, I::Misc(Misc::DI())),
            // Control flow instructions
            (0b1100_0011, I::ControlFlow(CF::JP(0))),
            (0b1100_1001, I::ControlFlow(CF::RET())),
            (0b1101_1001, I::ControlFlow(CF::RETI())),
            (0b1100_1101, I::ControlFlow(CF::CALL(0))),
            (0b1110_1001, I::ControlFlow(CF::JP_HL())),
            (0b0001_1000, I::ControlFlow(CF::JR(0))),
            (0b1100_0010, I::ControlFlow(CF::JPC(0, C::NotZero))),
            (0b1101_1010, I::ControlFlow(CF::JPC(0, C::Carry))),
            (0b1100_0000, I::ControlFlow(CF::RETC(C::NotZero))),
            (0b1101_1000, I::ControlFlow(CF::RETC(C::Carry))),
            (0b0010_0000, I::ControlFlow(CF::JRC(0, C::NotZero))),
            (0b0011_1000, I::ControlFlow(CF::JRC(0, C::Carry))),
            (0b1101_1100, I::ControlFlow(CF::CALLC(0, C::Carry))),
            (0b1100_0100, I::ControlFlow(CF::CALLC(0, C::NotZero))),
            (0b1100_0111, I::ControlFlow(CF::RST(0b1100_0111))),
            (0b1111_1111, I::ControlFlow(CF::RST(0b1111_1111))),
            // Load 8-bit instructions
            (0b0000_1010, I::Load8Bit(Load8Bit::LD_BC_TO_A())),
            (0b0001_1010, I::Load8Bit(Load8Bit::LD_DE_TO_A())),
            (0b0000_0010, I::Load8Bit(Load8Bit::LD_A_TO_BC())),
            (0b0001_0010, I::Load8Bit(Load8Bit::LD_A_TO_DE())),
            (0b1111_1010, I::Load8Bit(Load8Bit::LD_TO_A(0))),
            (0b1111_0010, I::Load8Bit(Load8Bit::LDH_C_TO_A())),
            (0b1110_0010, I::Load8Bit(Load8Bit::LDH_C_FROM_A())),
            (0b1111_0000, I::Load8Bit(Load8Bit::LDH_TO_A(0))),
            (0b1110_0000, I::Load8Bit(Load8Bit::LDH_FROM_A(0))),
            (0b1110_1010, I::Load8Bit(Load8Bit::LD_FROM_A(0))),
            (0b0011_1010, I::Load8Bit(Load8Bit::LD_A_FROM_HL_DEC())),
            (0b0011_0010, I::Load8Bit(Load8Bit::LD_A_TO_HL_DEC())),
            (0b0010_1010, I::Load8Bit(Load8Bit::LD_A_FROM_HL_INC())),
            (0b0010_0010, I::Load8Bit(Load8Bit::LD_A_TO_HL_INC())),
            (0b0100_0110, I::Load8Bit(Load8Bit::LD_FROM_HL(SR::B))),
            (0b0111_1110, I::Load8Bit(Load8Bit::LD_FROM_HL(SR::A))),
            (0b0111_0000, I::Load8Bit(Load8Bit::LD_TO_HL(SR::B))),
            (0b0111_0111, I::Load8Bit(Load8Bit::LD_TO_HL(SR::A))),
            (0b0110_0000, I::Load8Bit(Load8Bit::LD(SR::H, SR::B))),
            // Load 16-bit instructions
            (0b0000_0001, I::Load16Bit(Load16Bit::LD(DR::BC, 0))),
            (0b0001_0001, I::Load16Bit(Load16Bit::LD(DR::DE, 0))),
            (0b0010_0001, I::Load16Bit(Load16Bit::LD(DR::HL, 0))),
            (0b0011_0001, I::Load16Bit(Load16Bit::LD(DR::SP, 0))),
            (0b0000_1000, I::Load16Bit(Load16Bit::LD_FROM_SP(0))),
            (0b1111_1001, I::Load16Bit(Load16Bit::LD_HL_TO_SP())),
            (0b1100_0101, I::Load16Bit(Load16Bit::PUSH(DR::BC))),
            (0b1101_0101, I::Load16Bit(Load16Bit::PUSH(DR::DE))),
            (0b1110_0101, I::Load16Bit(Load16Bit::PUSH(DR::HL))),
            (0b1111_0101, I::Load16Bit(Load16Bit::PUSH(DR::AF))),
            (0b1100_0001, I::Load16Bit(Load16Bit::POP(DR::BC))),
            (0b1101_0001, I::Load16Bit(Load16Bit::POP(DR::DE))),
            (0b1110_0001, I::Load16Bit(Load16Bit::POP(DR::HL))),
            (0b1111_0001, I::Load16Bit(Load16Bit::POP(DR::AF))),
            // ALU 8-bit instructions
            (0b1000_0000, I::ALU8Bit(ALU8Bit::ADD(SR::B))),
            (0b1000_0001, I::ALU8Bit(ALU8Bit::ADD(SR::C))),
            (0b1000_0010, I::ALU8Bit(ALU8Bit::ADD(SR::D))),
            (0b1000_0011, I::ALU8Bit(ALU8Bit::ADD(SR::E))),
            (0b1000_0100, I::ALU8Bit(ALU8Bit::ADD(SR::H))),
            (0b1000_0101, I::ALU8Bit(ALU8Bit::ADD(SR::L))),
            (0b1000_0111, I::ALU8Bit(ALU8Bit::ADD(SR::A))),
            (0b1000_0110, I::ALU8Bit(ALU8Bit::ADD_HL())),
            (0b1100_0110, I::ALU8Bit(ALU8Bit::ADD_N(0))),
            (0b1000_1000, I::ALU8Bit(ALU8Bit::ADC(SR::B))),
            (0b1000_1001, I::ALU8Bit(ALU8Bit::ADC(SR::C))),
            (0b1000_1010, I::ALU8Bit(ALU8Bit::ADC(SR::D))),
            (0b1000_1011, I::ALU8Bit(ALU8Bit::ADC(SR::E))),
            (0b1000_1100, I::ALU8Bit(ALU8Bit::ADC(SR::H))),
            (0b1000_1101, I::ALU8Bit(ALU8Bit::ADC(SR::L))),
            (0b1000_1111, I::ALU8Bit(ALU8Bit::ADC(SR::A))),
            (0b1000_1110, I::ALU8Bit(ALU8Bit::ADC_HL())),
            (0b1100_1110, I::ALU8Bit(ALU8Bit::ADC_N(0))),
            (0b1001_0000, I::ALU8Bit(ALU8Bit::SUB(SR::B))),
            (0b1001_0001, I::ALU8Bit(ALU8Bit::SUB(SR::C))),
            (0b1001_0010, I::ALU8Bit(ALU8Bit::SUB(SR::D))),
            (0b1001_0011, I::ALU8Bit(ALU8Bit::SUB(SR::E))),
            (0b1001_0100, I::ALU8Bit(ALU8Bit::SUB(SR::H))),
            (0b1001_0101, I::ALU8Bit(ALU8Bit::SUB(SR::L))),
            (0b1001_0111, I::ALU8Bit(ALU8Bit::SUB(SR::A))),
            (0b1001_0110, I::ALU8Bit(ALU8Bit::SUB_HL())),
            (0b1101_0110, I::ALU8Bit(ALU8Bit::SUB_N(0))),
            (0b1001_1000, I::ALU8Bit(ALU8Bit::SBC(SR::B))),
            (0b1001_1001, I::ALU8Bit(ALU8Bit::SBC(SR::C))),
            (0b1001_1010, I::ALU8Bit(ALU8Bit::SBC(SR::D))),
            (0b1001_1011, I::ALU8Bit(ALU8Bit::SBC(SR::E))),
            (0b1001_1100, I::ALU8Bit(ALU8Bit::SBC(SR::H))),
            (0b1001_1101, I::ALU8Bit(ALU8Bit::SBC(SR::L))),
            (0b1001_1111, I::ALU8Bit(ALU8Bit::SBC(SR::A))),
            (0b1001_1110, I::ALU8Bit(ALU8Bit::SBC_HL())),
            (0b1101_1110, I::ALU8Bit(ALU8Bit::SBC_N(0))),
            (0b1010_0000, I::ALU8Bit(ALU8Bit::AND(SR::B))),
            (0b1010_0001, I::ALU8Bit(ALU8Bit::AND(SR::C))),
            (0b1010_0010, I::ALU8Bit(ALU8Bit::AND(SR::D))),
            (0b1010_0011, I::ALU8Bit(ALU8Bit::AND(SR::E))),
            (0b1010_0100, I::ALU8Bit(ALU8Bit::AND(SR::H))),
            (0b1010_0101, I::ALU8Bit(ALU8Bit::AND(SR::L))),
            (0b1010_0111, I::ALU8Bit(ALU8Bit::AND(SR::A))),
            (0b1010_0110, I::ALU8Bit(ALU8Bit::AND_HL())),
            (0b1110_0110, I::ALU8Bit(ALU8Bit::AND_N(0))),
            (0b1011_0000, I::ALU8Bit(ALU8Bit::OR(SR::B))),
            (0b1011_0001, I::ALU8Bit(ALU8Bit::OR(SR::C))),
            (0b1011_0010, I::ALU8Bit(ALU8Bit::OR(SR::D))),
            (0b1011_0011, I::ALU8Bit(ALU8Bit::OR(SR::E))),
            (0b1011_0100, I::ALU8Bit(ALU8Bit::OR(SR::H))),
            (0b1011_0101, I::ALU8Bit(ALU8Bit::OR(SR::L))),
            (0b1011_0111, I::ALU8Bit(ALU8Bit::OR(SR::A))),
            (0b1011_0110, I::ALU8Bit(ALU8Bit::OR_HL())),
            (0b1111_0110, I::ALU8Bit(ALU8Bit::OR_N(0))),
            (0b1010_1000, I::ALU8Bit(ALU8Bit::XOR(SR::B))),
            (0b1010_1001, I::ALU8Bit(ALU8Bit::XOR(SR::C))),
            (0b1010_1010, I::ALU8Bit(ALU8Bit::XOR(SR::D))),
            (0b1010_1011, I::ALU8Bit(ALU8Bit::XOR(SR::E))),
            (0b1010_1100, I::ALU8Bit(ALU8Bit::XOR(SR::H))),
            (0b1010_1101, I::ALU8Bit(ALU8Bit::XOR(SR::L))),
            (0b1010_1111, I::ALU8Bit(ALU8Bit::XOR(SR::A))),
            (0b1010_1110, I::ALU8Bit(ALU8Bit::XOR_HL())),
            (0b1110_1110, I::ALU8Bit(ALU8Bit::XOR_N(0))),
            (0b1011_1000, I::ALU8Bit(ALU8Bit::CP(SR::B))),
            (0b1011_1001, I::ALU8Bit(ALU8Bit::CP(SR::C))),
            (0b1011_1010, I::ALU8Bit(ALU8Bit::CP(SR::D))),
            (0b1011_1011, I::ALU8Bit(ALU8Bit::CP(SR::E))),
            (0b1011_1100, I::ALU8Bit(ALU8Bit::CP(SR::H))),
            (0b1011_1101, I::ALU8Bit(ALU8Bit::CP(SR::L))),
            (0b1011_1111, I::ALU8Bit(ALU8Bit::CP(SR::A))),
            (0b1011_1110, I::ALU8Bit(ALU8Bit::CP_HL())),
            (0b1111_1110, I::ALU8Bit(ALU8Bit::CP_N(0))),
            (0b0000_0100, I::ALU8Bit(ALU8Bit::INC(SR::B))),
            (0b0000_1100, I::ALU8Bit(ALU8Bit::INC(SR::C))),
            (0b0001_0100, I::ALU8Bit(ALU8Bit::INC(SR::D))),
            (0b0001_1100, I::ALU8Bit(ALU8Bit::INC(SR::E))),
            (0b0010_0100, I::ALU8Bit(ALU8Bit::INC(SR::H))),
            (0b0010_1100, I::ALU8Bit(ALU8Bit::INC(SR::L))),
            (0b0011_0100, I::ALU8Bit(ALU8Bit::INC_HL())),
            (0b0011_1100, I::ALU8Bit(ALU8Bit::INC(SR::A))),
            (0b0000_0101, I::ALU8Bit(ALU8Bit::DEC(SR::B))),
            (0b0000_1101, I::ALU8Bit(ALU8Bit::DEC(SR::C))),
            (0b0001_0101, I::ALU8Bit(ALU8Bit::DEC(SR::D))),
            (0b0001_1101, I::ALU8Bit(ALU8Bit::DEC(SR::E))),
            (0b0010_0101, I::ALU8Bit(ALU8Bit::DEC(SR::H))),
            (0b0010_1101, I::ALU8Bit(ALU8Bit::DEC(SR::L))),
            (0b0011_0101, I::ALU8Bit(ALU8Bit::DEC_HL())),
            (0b0011_1101, I::ALU8Bit(ALU8Bit::DEC(SR::A))),
            // ALU 16-bit instructions
            (0b0000_1001, I::ALU16Bit(ALU16Bit::ADD_HL(DR::BC))),
            (0b0001_1001, I::ALU16Bit(ALU16Bit::ADD_HL(DR::DE))),
            (0b0010_1001, I::ALU16Bit(ALU16Bit::ADD_HL(DR::HL))),
            (0b0011_1001, I::ALU16Bit(ALU16Bit::ADD_HL(DR::SP))),
            (0b1110_1000, I::ALU16Bit(ALU16Bit::ADD_SP(0))),
            (0b0000_0011, I::ALU16Bit(ALU16Bit::INC(DR::BC))),
            (0b0001_0011, I::ALU16Bit(ALU16Bit::INC(DR::DE))),
            (0b0010_0011, I::ALU16Bit(ALU16Bit::INC(DR::HL))),
            (0b0011_0011, I::ALU16Bit(ALU16Bit::INC(DR::SP))),
            (0b0000_1011, I::ALU16Bit(ALU16Bit::DEC(DR::BC))),
            (0b0001_1011, I::ALU16Bit(ALU16Bit::DEC(DR::DE))),
            (0b0010_1011, I::ALU16Bit(ALU16Bit::DEC(DR::HL))),
            (0b0011_1011, I::ALU16Bit(ALU16Bit::DEC(DR::SP))),
            // Rotate Shift instructions
            (0b0000_0111, I::RotateShift(RS::RLCA())),
            (0b0000_1111, I::RotateShift(RS::RRCA())),
            (0b0001_0111, I::RotateShift(RS::RLA())),
            (0b0001_1111, I::RotateShift(RS::RRA())),
        ] {
            assert_eq!(
                instruction,
                decode(code, pc, &memory).unwrap(),
                "Failed to decode 0b{code:08b}"
            );
        }
    }
}
