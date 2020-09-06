use super::{Condition, Instruction, InstructionResult};
use crate::registers::DoubleRegister;
use std::fmt::Display;

/// Unconditional jump to location specified by 16-bit operand.
///
/// ## Examples
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = Jp { operand: 0xBADA };
///
/// assert_eq!(0, registers.pc);
/// instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0xBADA, registers.pc);
/// ```
pub struct Jp {
    pub operand: u16,
}

impl Instruction for Jp {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> InstructionResult {
        registers.pc = self.operand;
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
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = JpIf { operand: 0xBADA, condition: Condition::Carry };
///
/// let mut cycles = instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0, registers.pc);
/// assert_eq!(3, cycles);
///
/// registers.set_single(SingleRegister::F, 0b0001_0000);
/// cycles = instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0xBADA, registers.pc);
/// assert_eq!(4, cycles);
/// ```
pub struct JpIf {
    pub condition: Condition,
    pub operand: u16,
}

impl Instruction for JpIf {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> InstructionResult {
        if self.condition.is_fulfilled(registers) {
            registers.pc = self.operand;
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
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = JpToHL { };
///
/// registers.set_double(DoubleRegister::HL, 0xBADA);
/// instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0xBADA, registers.pc);
/// ```
pub struct JpToHL {}

impl Instruction for JpToHL {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> InstructionResult {
        registers.pc = registers.get_double(DoubleRegister::HL);
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
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// registers.pc = 0x0200;
/// let instruction = JpToOffset { operand: 0x42 };
///
/// instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0x0242, registers.pc);
/// ```
pub struct JpToOffset {
    pub operand: u8,
}

impl Instruction for JpToOffset {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> InstructionResult {
        registers.pc += self.operand as u16;
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
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = JpToOffsetIf { operand: 0x42, condition: Condition::Zero };
/// let mut cycles = 0;
/// registers.pc = 0x0200;
///
/// cycles = instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0x0200, registers.pc);
/// assert_eq!(2, cycles);
///
/// registers.set_single(SingleRegister::F, 0b1000_0000);
///
/// cycles = instruction.execute(&mut registers, &mut memory).unwrap();
/// assert_eq!(0x0242, registers.pc);
/// assert_eq!(3, cycles);
/// ```
pub struct JpToOffsetIf {
    pub operand: u8,
    pub condition: Condition,
}

impl Instruction for JpToOffsetIf {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        _memory: &mut crate::memory::Memory,
    ) -> InstructionResult {
        if self.condition.is_fulfilled(registers) {
            registers.pc += self.operand as u16;
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
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = Call { operand: 0xABCD };
/// registers.pc = 0xAAAA;
/// instruction.execute(&mut registers, &mut memory).unwrap();
///
/// assert_eq!(0xABCD, registers.pc);
/// assert_eq!(0xFFFC, registers.sp);
/// assert_eq!(0xAAAA, memory.get_u16(registers.sp.into()));
/// ```
pub struct Call {
    pub operand: u16,
}

impl Instruction for Call {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        memory: &mut crate::memory::Memory,
    ) -> InstructionResult {
        let [lo, hi] = registers.pc.to_le_bytes();
        memory.set((registers.sp - 1).into(), hi);
        memory.set((registers.sp - 2).into(), lo);
        registers.sp -= 2;
        registers.pc = self.operand;
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
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = CallIf { operand: 0xABCD, condition: Condition::Carry };
/// registers.pc = 0xAAAA;
///
/// let cycles = instruction.execute(&mut registers, &mut memory).unwrap();
///
/// assert_eq!(registers.pc, 0xAAAA);
/// assert_eq!(registers.sp, 0xFFFE);
/// assert_eq!(3, cycles);
/// ```
///
/// If the condition is fulfilled then the function is called:
///
/// ```
/// # use gejmboj_cpu::registers::*;
/// # use gejmboj_cpu::memory::*;
/// # use gejmboj_cpu::instructions::*;
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let instruction = CallIf { operand: 0xABCD, condition: Condition::Carry };
/// registers.pc = 0xAAAA;
/// registers.set_single(SingleRegister::F, 0b0001_0000);
///
/// let cycles = instruction.execute(&mut registers, &mut memory).unwrap();
///
/// assert_eq!(registers.pc, 0xABCD);
/// assert_eq!(registers.sp, 0xFFFC);
/// assert_eq!(6, cycles);
/// ```
pub struct CallIf {
    pub operand: u16,
    pub condition: Condition,
}

impl Instruction for CallIf {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        memory: &mut crate::memory::Memory,
    ) -> InstructionResult {
        if self.condition.is_fulfilled(registers) {
            let [lo, hi] = registers.pc.to_le_bytes();
            memory.set((registers.sp - 1).into(), hi);
            memory.set((registers.sp - 2).into(), lo);
            registers.sp -= 2;
            registers.pc = self.operand;
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
/// let mut registers = Registers::new();
/// let mut memory = Memory::new();
/// let function_call = Call { operand: 0xABCD };
/// let return_call = Ret { };
///
/// registers.pc = 0xAAAA;
/// function_call.execute(&mut registers, &mut memory).unwrap();
/// return_call.execute(&mut registers, &mut memory).unwrap();
///
/// assert_eq!(0xAAAA, registers.pc);
/// assert_eq!(0xFFFE, registers.sp);
/// ```
pub struct Ret {}

impl Instruction for Ret {
    fn execute(
        &self,
        registers: &mut crate::registers::Registers,
        memory: &mut crate::memory::Memory,
    ) -> InstructionResult {
        registers.pc = memory.get_u16(registers.sp.into());
        registers.sp += 2;
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
