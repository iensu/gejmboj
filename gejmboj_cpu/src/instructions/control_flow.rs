use super::{Condition, Instruction, InstructionResult};
use crate::{
    cpu::CpuFlags,
    memory,
    registers::{self, DoubleRegister},
};
use memory::Memory;
use registers::Registers;
use std::fmt::Display;

/// Unconditional jump to location specified by 16-bit operand.
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let instruction = Jp { operand: 0xBADA };
///
/// assert_eq!(0, registers.PC);
/// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// assert_eq!(0xBADA, registers.PC);
/// ```
pub struct Jp {
    pub operand: u16,
}

impl Instruction for Jp {
    fn execute(
        &self,
        registers: &mut Registers,
        _memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        registers.PC = self.operand;
        Ok(4)
    }

    fn length(&self) -> u16 {
        3
    }
}

impl Display for Jp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP({:04x})", self.operand)
    }
}

/// Conditional jump to location specified by 16-bit operand.
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let instruction = JpIf { operand: 0xBADA, condition: Condition::Carry };
///
/// let mut cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// assert_eq!(0, registers.PC);
/// assert_eq!(3, cycles);
///
/// registers.set_single(SingleRegister::F, 0b0001_0000);
/// cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// assert_eq!(0xBADA, registers.PC);
/// assert_eq!(4, cycles);
/// ```
pub struct JpIf {
    pub condition: Condition,
    pub operand: u16,
}

impl Instruction for JpIf {
    fn execute(
        &self,
        registers: &mut Registers,
        _memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        if self.condition.is_fulfilled(registers) {
            registers.PC = self.operand;
            Ok(4)
        } else {
            Ok(3)
        }
    }

    fn length(&self) -> u16 {
        3
    }
}

impl Display for JpIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP({:04x}) {:?}", self.operand, self.condition)
    }
}

/// Unconditional jump to location specified by register HL
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let instruction = JpToHL { };
///
/// registers.set_double(DoubleRegister::HL, 0xBADA);
/// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// assert_eq!(0xBADA, registers.PC);
/// ```
pub struct JpToHL {}

impl Instruction for JpToHL {
    fn execute(
        &self,
        registers: &mut Registers,
        _memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        registers.PC = registers.get_double(DoubleRegister::HL);
        Ok(1)
    }

    fn length(&self) -> u16 {
        1
    }
}

impl Display for JpToHL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP(HL)")
    }
}

/// Unconditional jump to location at current + offset
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// registers.PC = 0x0200;
/// let instruction = JpToOffset { operand: 0x42 };
///
/// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// assert_eq!(0x0242, registers.PC);
/// ```
pub struct JpToOffset {
    pub operand: u8,
}

impl Instruction for JpToOffset {
    fn execute(
        &self,
        registers: &mut Registers,
        _memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        registers.PC += self.operand as u16;
        Ok(3)
    }

    fn length(&self) -> u16 {
        2
    }
}

impl Display for JpToOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP(PC + {:02x})", self.operand)
    }
}

/// Coonditional jump to relative address specified by offset operand.
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let instruction = JpToOffsetIf { operand: 0x42, condition: Condition::Zero };
/// let mut cycles = 0;
/// registers.PC = 0x0200;
///
/// cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// assert_eq!(0x0200, registers.PC);
/// assert_eq!(2, cycles);
///
/// registers.set_single(SingleRegister::F, 0b1000_0000);
///
/// cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// assert_eq!(0x0242, registers.PC);
/// assert_eq!(3, cycles);
/// ```
pub struct JpToOffsetIf {
    pub operand: u8,
    pub condition: Condition,
}

impl Instruction for JpToOffsetIf {
    fn execute(
        &self,
        registers: &mut Registers,
        _memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        if self.condition.is_fulfilled(registers) {
            registers.PC += self.operand as u16;
            Ok(3)
        } else {
            Ok(2)
        }
    }

    fn length(&self) -> u16 {
        2
    }
}

impl Display for JpToOffsetIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JP(PC + {:02x}) {:?}", self.operand, self.condition)
    }
}

/// Unconditional call of the function at operand address.
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let instruction = Call { operand: 0xABCD };
/// registers.PC = 0xAAAA;
/// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(0xABCD, registers.PC);
/// assert_eq!(0xFFFC, registers.SP);
/// assert_eq!(0xAAAA, memory.get_u16(registers.SP.into()));
/// ```
pub struct Call {
    pub operand: u16,
}

impl Instruction for Call {
    fn execute(
        &self,
        registers: &mut Registers,
        memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        let [lo, hi] = registers.PC.to_le_bytes();
        memory.set((registers.SP - 1).into(), hi);
        memory.set((registers.SP - 2).into(), lo);
        registers.SP -= 2;
        registers.PC = self.operand;
        Ok(6)
    }

    fn length(&self) -> u16 {
        3
    }
}

impl Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CALL({:04x})", self.operand)
    }
}

/// Conditional function call.
///
/// ## Examples
///
/// Function is not called if the condition is not fulfilled:
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let instruction = CallIf { operand: 0xABCD, condition: Condition::Carry };
/// registers.PC = 0xAAAA;
///
/// let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(registers.PC, 0xAAAA);
/// assert_eq!(registers.SP, 0xFFFE);
/// assert_eq!(3, cycles);
/// ```
///
/// If the condition is fulfilled then the function is called:
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let instruction = CallIf { operand: 0xABCD, condition: Condition::Carry };
/// registers.PC = 0xAAAA;
/// registers.set_single(SingleRegister::F, 0b0001_0000);
///
/// let cycles = instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(registers.PC, 0xABCD);
/// assert_eq!(registers.SP, 0xFFFC);
/// assert_eq!(6, cycles);
/// ```
pub struct CallIf {
    pub operand: u16,
    pub condition: Condition,
}

impl Instruction for CallIf {
    fn execute(
        &self,
        registers: &mut Registers,
        memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        if self.condition.is_fulfilled(registers) {
            let [lo, hi] = registers.PC.to_le_bytes();
            memory.set((registers.SP - 1).into(), hi);
            memory.set((registers.SP - 2).into(), lo);
            registers.SP -= 2;
            registers.PC = self.operand;
            Ok(6)
        } else {
            Ok(3)
        }
    }

    fn length(&self) -> u16 {
        3
    }
}

impl Display for CallIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CALL({:04x}) {:?}", self.operand, self.condition)
    }
}

/// Unconditional return from function.
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let function_call = Call { operand: 0xABCD };
/// let return_call = Ret { };
///
/// registers.PC = 0xAAAA;
/// function_call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// return_call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(0xAAAA, registers.PC);
/// assert_eq!(0xFFFE, registers.SP);
/// ```
pub struct Ret {}

impl Instruction for Ret {
    fn execute(
        &self,
        registers: &mut Registers,
        memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        registers.PC = memory.get_u16(registers.SP.into());
        registers.SP += 2;
        Ok(4)
    }

    fn length(&self) -> u16 {
        1
    }
}

impl Display for Ret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RET")
    }
}

/// Conditionally return from function.
///
/// ## Examples
///
/// Return from function call only if condition is fulfilled:
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// registers.PC = 0xAAAA;
///
/// let call = Call { operand: 0xABCD };
/// call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// let ret = RetIf { condition: Condition::Carry };
/// let mut cycles = 0;
/// cycles = ret.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(0xABCD, registers.PC);
/// assert_eq!(0xFFFC, registers.SP);
/// assert_eq!(2, cycles);
///
/// registers.set_single(SingleRegister::F, 0b0001_0000);
/// cycles = ret.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(0xAAAA, registers.PC);
/// assert_eq!(0xFFFE, registers.SP);
/// assert_eq!(5, cycles);
/// ```
pub struct RetIf {
    pub condition: Condition,
}

impl Instruction for RetIf {
    fn execute(
        &self,
        registers: &mut Registers,
        memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        if self.condition.is_fulfilled(registers) {
            registers.PC = memory.get_u16(registers.SP.into());
            registers.SP += 2;
            Ok(5)
        } else {
            Ok(2)
        }
    }

    fn length(&self) -> u16 {
        1
    }
}

impl Display for RetIf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RET {:?}", self.condition)
    }
}

/// Unconditional return from a function which enables interrupts
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let call = Call { operand: 0xABCD };
/// let reti = RetI { };
///
/// registers.PC = 0xAAAA;
/// call.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
/// reti.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(0xAAAA, registers.PC);
/// assert_eq!(0xFFFE, registers.SP);
/// assert_eq!(1, cpu_flags.IME);
/// ```
pub struct RetI {}

impl Instruction for RetI {
    fn execute(
        &self,
        registers: &mut Registers,
        memory: &mut Memory,
        cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        registers.PC = memory.get_u16(registers.SP.into());
        registers.SP += 2;
        cpu_flags.IME = 1;
        Ok(4)
    }

    fn length(&self) -> u16 {
        1
    }
}

impl Display for RetI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RET IME=1")
    }
}

/// Unconditional function call to the RESET address defined by bits 3-5
///
/// Possible RESET addresses are:
///
/// * `0x00`
/// * `0x08`
/// * `0x10`
/// * `0x18`
/// * `0x20`
/// * `0x28`
/// * `0x30`
/// * `0x38`
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// # use gejmboj_cpu::cpu::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let mut cpu_flags = CpuFlags::new();
/// let instruction = Rst { opcode: 0b1101_0111};
///
/// instruction.execute(&mut registers, &mut memory, &mut cpu_flags).unwrap();
///
/// assert_eq!(0x10, registers.PC);
/// ```
pub struct Rst {
    pub opcode: u8,
}

impl Instruction for Rst {
    fn execute(
        &self,
        registers: &mut Registers,
        _memory: &mut Memory,
        _cpu_flags: &mut CpuFlags,
    ) -> InstructionResult {
        registers.PC = get_reset_address(self.opcode);
        Ok(4)
    }

    fn length(&self) -> u16 {
        1
    }
}

impl Display for Rst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RST({:02x})", get_reset_address(self.opcode))
    }
}

fn get_reset_address(opcode: u8) -> u16 {
    (opcode & 0b00111000) as u16
}
